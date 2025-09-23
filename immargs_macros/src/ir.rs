#![doc(hidden)]

use crate::ast::*;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::collections::HashMap;
use std::collections::HashSet;
use syn::Error;
use syn::Ident;
use syn::Result;

pub struct Ir {
    pub ident: Ident,
    pub options: Vec<IrOption>,
    pub non_options: Vec<IrNonOption>,
}

pub struct IrOption {
    pub kind: IrOptionKind,
    pub variadic: bool,
    pub field: Ident,
    pub shorts: Vec<char>,
    pub longs: Vec<String>,
    pub conflicts: Vec<String>,
    pub usage: String,
    pub help: Option<String>,
}

pub enum IrOptionKind {
    NoValue,
    Value(TokenStream),
    Version,
    Help,
}

pub struct IrNonOption {
    pub kind: IrNonOptionKind,
    pub optional: bool,
    pub variadic: bool,
    pub field: Ident,
    pub name: String,
    pub conflicts: Vec<String>,
    pub usage: String,
    pub help: Option<String>,
}

pub enum IrNonOptionKind {
    Value(TokenStream),
    Command((TokenStream, Vec<IrCommand>)),
}

pub struct IrCommand {
    pub names: Vec<String>,
    pub usage: String,
    pub help: Option<String>,
}

macro_rules! bail {
    ($span:expr, $msg:expr) => {
        return Err(Error::new($span, $msg))
    };
}

const VERSION: &str = "version";
const HELP: &str = "help";

pub fn lower(ast: Ast) -> Result<Ir> {
    let mut verify = Verify::default();

    verify_conflicts(&ast)?;
    verify_help(&ast)?;

    let ident = ident(&ast);
    let options = options(&ast, &mut verify)?;
    let non_options = non_options(&ast, &mut verify)?;

    let ir = Ir {
        ident,
        options,
        non_options,
    };

    Ok(ir)
}

fn normalize_underscore(ident: &Ident) -> String {
    ident
        .to_string()
        .trim_start_matches('_')
        .trim_end_matches('_')
        .replace("_", "-")
}

fn normalize_ident(ident: &Ident) -> String {
    normalize_underscore(ident).to_lowercase()
}

fn ident(ast: &Ast) -> Ident {
    ast.ident.clone().unwrap_or(format_ident!("ImmArgs"))
}

fn options(ast: &Ast, verify: &mut Verify) -> Result<Vec<IrOption>> {
    let mut options = vec![];
    let mut allow_option = true;

    for arg in &ast.arguments.0 {
        let AstArgument::Option(arg) = arg else {
            allow_option = false;
            continue;
        };

        if !allow_option {
            bail!(arg.span, "cannot have option after non-option");
        }

        let option = IrOption {
            kind: option_kind(arg),
            variadic: option_variadic(arg),
            field: option_field(arg, verify)?,
            shorts: option_shorts(arg, verify)?,
            longs: option_longs(arg, verify)?,
            conflicts: option_conflicts(arg),
            usage: option_usage(arg),
            help: option_help(arg),
        };

        options.push(option_special(arg, option)?);
    }

    Ok(options)
}

fn option_kind(arg: &AstOption) -> IrOptionKind {
    match &arg.value.0 {
        None => IrOptionKind::NoValue,
        Some((_, ty)) => IrOptionKind::Value(quote! { #ty }),
    }
}

fn option_variadic(arg: &AstOption) -> bool {
    arg.variadic.0.is_some()
}

fn option_field(arg: &AstOption, verify: &mut Verify) -> Result<Ident> {
    let first_long = arg.longs.0.first().map(|long| long.0.clone());
    let first_short = arg.shorts.0.first().map(|short| short.0.clone());

    let ident = match (first_long, first_short) {
        (Some(ident), _) => ident,
        (_, Some(ident)) => ident,
        _ => unreachable!(),
    };

    verify.unique_field(&ident)?;
    Ok(ident)
}

fn option_shorts(arg: &AstOption, verify: &mut Verify) -> Result<Vec<char>> {
    let mut shorts = vec![];

    for short in &arg.shorts.0 {
        let span = short.0.span();
        let short = normalize_underscore(&short.0);

        if short.chars().count() != 1 {
            bail!(span, "expected single alphanumeric character");
        }

        let short = short.chars().next().unwrap();

        verify.unique_short(short, span)?;
        shorts.push(short);
    }

    Ok(shorts)
}

fn option_longs(arg: &AstOption, verify: &mut Verify) -> Result<Vec<String>> {
    let mut longs = vec![];

    for long in &arg.longs.0 {
        let span = long.0.span();
        let long = normalize_ident(&long.0);

        if long.chars().count() < 2 {
            bail!(span, "normalized long option must be at least 2 characters");
        }

        verify.unique_long(&long, span)?;
        longs.push(long);
    }

    Ok(longs)
}

fn option_conflicts(arg: &AstOption) -> Vec<String> {
    let mut conflicts = vec![];

    for AstConflict(kind, _, id) in &arg.conflicts.0 {
        let id = id
            .as_ref()
            .map(|conflict| conflict.to_string())
            .unwrap_or_default();
        conflicts.push(format!("{kind}{id}"));
    }

    conflicts
}

fn option_usage(arg: &AstOption) -> String {
    let mut usage = vec![];

    for short in &arg.shorts.0 {
        usage.push(format!("-{}", normalize_underscore(&short.0)));
    }

    for long in &arg.longs.0 {
        usage.push(format!("--{}", normalize_ident(&long.0)));
    }

    if let Some((value, _)) = &arg.value.0 {
        let last = usage.last_mut().unwrap();
        last.push_str(&format!(" <{}>", normalize_underscore(value)));
    }

    usage.join(", ")
}

fn option_help(arg: &AstOption) -> Option<String> {
    arg.help.0.as_ref().map(|help| help.value())
}

fn option_special(arg: &AstOption, mut option: IrOption) -> Result<IrOption> {
    for (special_long, special_kind) in
        [(VERSION, IrOptionKind::Version), (HELP, IrOptionKind::Help)]
    {
        if option.longs.iter().any(|long| long == special_long) {
            if !matches!(option.kind, IrOptionKind::NoValue)
                || option.variadic
                || !option.conflicts.is_empty()
            {
                bail!(
                    arg.span,
                    format!(
                        "special option --{special_long} cannot take a value, \
                        be variadic, or have conflcits"
                    )
                );
            }

            option.kind = special_kind;
        }
    }

    Ok(option)
}

fn non_options(ast: &Ast, verify: &mut Verify) -> Result<Vec<IrNonOption>> {
    let mut non_options = vec![];
    let mut has_variadic = false;
    let mut has_command = false;

    for arg in &ast.arguments.0 {
        let AstArgument::NonOption(arg) = arg else {
            continue;
        };

        let is_variadic = arg.variadic.0.is_some();
        let is_command = arg.commands.0.is_some();

        if is_variadic && is_command {
            bail!(arg.span, "command argument cannot be variadic");
        }

        if has_variadic & is_variadic {
            bail!(arg.span, "cannot have multiple variadic arguments");
        }

        if has_variadic && is_command {
            bail!(arg.span, "command argument cannot follow variadic argument")
        }

        if has_command {
            bail!(arg.span, "arguments cannot follow command argument")
        }

        has_variadic |= is_variadic;
        has_command |= is_command;

        let non_option = IrNonOption {
            kind: non_option_kind(arg),
            optional: non_option_optional(arg),
            variadic: non_option_variadic(arg),
            field: non_option_field(arg, verify)?,
            name: non_option_name(arg),
            conflicts: non_option_conflicts(arg),
            usage: non_option_usage(arg),
            help: non_option_help(arg),
        };

        non_options.push(non_option);
    }

    Ok(non_options)
}

fn non_option_kind(arg: &AstNonOption) -> IrNonOptionKind {
    let ty = &arg.ty;

    let Some(commands) = &arg.commands.0 else {
        return IrNonOptionKind::Value(quote! { #ty });
    };

    let mut cmds = vec![];

    for command in commands {
        let names = command
            .idents
            .iter()
            .map(normalize_underscore)
            .collect::<Vec<_>>();
        let help = command.help.0.as_ref().map(|help| help.value());
        let usage = names.join(", ");
        let command = IrCommand { names, usage, help };
        cmds.push(command)
    }

    IrNonOptionKind::Command((quote! { #ty }, cmds))
}

fn non_option_optional(arg: &AstNonOption) -> bool {
    arg.optional
}

fn non_option_variadic(arg: &AstNonOption) -> bool {
    arg.variadic.0.is_some()
}

fn non_option_field(arg: &AstNonOption, verify: &mut Verify) -> Result<Ident> {
    let ident = &arg.ident;
    verify.unique_field(ident)?;
    Ok(ident.clone())
}

fn non_option_name(arg: &AstNonOption) -> String {
    let name = normalize_ident(&arg.ident);
    format!("<{name}>")
}

fn non_option_conflicts(arg: &AstNonOption) -> Vec<String> {
    let mut conflicts = vec![];

    for AstConflict(kind, _, id) in &arg.conflicts.0 {
        let id = id
            .as_ref()
            .map(|conflict| conflict.to_string())
            .unwrap_or_default();
        conflicts.push(format!("{kind}{id}"));
    }

    conflicts
}

fn non_option_usage(arg: &AstNonOption) -> String {
    let name = normalize_ident(&arg.ident);

    match (arg.optional, arg.variadic.0) {
        (true, None) => format!("[<{name}>]"),
        (true, Some(_)) => format!("[<{name}>...]"),
        (false, None) => format!("<{name}>"),
        (false, Some(_)) => format!("<{name}>..."),
    }
}

fn non_option_help(arg: &AstNonOption) -> Option<String> {
    arg.help.0.as_ref().map(|help| help.value())
}

fn verify_conflicts(ast: &Ast) -> Result<()> {
    let mut conflict_map = HashMap::new();

    for arg in &ast.arguments.0 {
        let (non_option, conflicts) = match arg {
            AstArgument::Option(option) => (false, &option.conflicts.0),
            AstArgument::NonOption(non_option) => (true, &non_option.conflicts.0),
        };

        for AstConflict(kind, span, id) in conflicts {
            let id = id
                .as_ref()
                .map(|id| id.to_string())
                .unwrap_or(String::new());
            let id = format!("{kind}{id}");
            conflict_map
                .entry(id)
                .or_insert(vec![])
                .push((*span, non_option));
        }
    }

    if conflict_map.len() == 1 {
        let (id, spans) = conflict_map.iter().next().unwrap();
        if id.chars().count() != 1 {
            let span = spans[0].0;
            bail!(span, "explicit conflict-id not needed");
        }
    }

    for (_, spans) in conflict_map {
        if spans.len() == 1 {
            let span = spans[0].0;
            bail!(span, "conflict has no effect");
        }

        let mut num_non_options = 0;
        for (span, non_option) in spans {
            if non_option {
                num_non_options += 1;
            }

            if num_non_options > 1 {
                bail!(span, "non-options cannot conflict with each other");
            }
        }
    }

    Ok(())
}

fn verify_help(ast: &Ast) -> Result<()> {
    let mut help_option = false;
    let mut help_message = None;

    for arg in &ast.arguments.0 {
        if let AstArgument::Option(option) = arg {
            help_option |= option.longs.0.iter().any(|long| long.0 == HELP);
            if help_message.is_none() {
                help_message = option.help.0.as_ref();
            }
        }
    }

    if !help_option && let Some(message) = help_message {
        bail!(
            message.span(),
            "help message without --help option has no effect"
        );
    }

    Ok(())
}

#[derive(Default)]
struct Verify {
    shorts: HashSet<char>,
    longs: HashSet<String>,
    fields: HashSet<String>,
}

impl Verify {
    fn unique_short(&mut self, short: char, span: Span) -> Result<()> {
        if !self.shorts.insert(short) {
            bail!(span, "conflicts with previously defined short option");
        }

        Ok(())
    }

    fn unique_long(&mut self, long: &str, span: Span) -> Result<()> {
        if !self.longs.insert(long.to_string()) {
            bail!(span, "conflicts with previously defined long option");
        }

        Ok(())
    }

    fn unique_field(&mut self, ident: &Ident) -> Result<()> {
        if !self.fields.insert(ident.to_string()) {
            bail!(ident.span(), "conflicts with previously defined argument");
        }

        Ok(())
    }
}
