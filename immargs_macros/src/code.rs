#![doc(hidden)]

use super::ir::*;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Ident;
use syn::Result;

macro_rules! code {
    ($($code:tt)*) => {
        Some(quote! { $($code)* })
    };
}

pub fn emit(ir: Ir) -> Result<TokenStream> {
    let mut declare_options = vec![];
    let mut declare_non_options = vec![];
    let mut declare_fields = vec![];
    let mut assign_fields = vec![];
    let mut setters_options = vec![];
    let mut setters_non_options = vec![];

    for arg in &ir.options {
        let kind = &arg.kind;
        let variadic = &arg.variadic;
        let field = &arg.field;
        let shorts = &arg.shorts;
        let longs = &arg.longs;
        let conflicts = &arg.conflicts;
        let variable = format_ident!("option_{field}");

        let field_ty = match (kind, variadic) {
            (IrOptionKind::NoValue, false) => code! { bool },
            (IrOptionKind::NoValue, true) => code! { usize },
            (IrOptionKind::Value(ty), false) => code! { Option<#ty> },
            (IrOptionKind::Value(ty), true) => code! { Vec<#ty> },
            _ => None,
        };

        let names = {
            let shorts = shorts
                .iter()
                .map(|short| format!("-{short}"))
                .collect::<Vec<_>>();

            let longs = longs
                .iter()
                .map(|long| format!("--{long}"))
                .collect::<Vec<_>>();

            [&shorts[..], &longs[..]].concat()
        };

        let build_value_or_version_or_help = match kind {
            IrOptionKind::Value(ty) => code! { .value::<#ty>() },
            IrOptionKind::Version => code! { .version(name, version) },
            IrOptionKind::Help => code! { .help(&help) },
            _ => None,
        };

        let build_variadic = match variadic {
            true => code! { .variadic() },
            _ => None,
        };

        let build_conflicts = match conflicts.is_empty() {
            false => code! { .conflicts(&[#(#conflicts),*]) },
            _ => None,
        };

        declare_options.push(code! {
            let mut #variable = __private::option(&[#(#names),*])
                #build_value_or_version_or_help
                #build_variadic
                #build_conflicts
                ;
        });

        declare_fields.push(match kind {
            IrOptionKind::Version => None,
            IrOptionKind::Help => None,
            _ => code! { pub #field: #field_ty, },
        });

        assign_fields.push(match kind {
            IrOptionKind::Version => None,
            IrOptionKind::Help => None,
            _ => code! { #field: #variable.into(), },
        });

        setters_options.push(code! { #variable.as_setter(), });
    }

    for arg in &ir.non_options {
        let kind = &arg.kind;
        let optional = &arg.optional;
        let variadic = &arg.variadic;
        let field = &arg.field;
        let name = &arg.name;
        let conflicts = &arg.conflicts;
        let variable = format_ident!("non_option_{field}");

        let field_ty = match (kind, optional, variadic) {
            (IrNonOptionKind::Value(ty), false, false) => code! { #ty },
            (IrNonOptionKind::Value(ty), true, false) => code! { Option<#ty> },
            (IrNonOptionKind::Value(ty), _, true) => code! { Vec<#ty> },
            (IrNonOptionKind::Command((ty, _)), false, _) => code! { #ty },
            (IrNonOptionKind::Command((ty, _)), true, _) => code! { Option<#ty> },
        };

        let build_value_or_command = match kind {
            IrNonOptionKind::Value(ty) => code! { .value::<#ty>() },
            IrNonOptionKind::Command((ty, _)) => code! { .command::<#ty>() },
        };

        let build_optional = match optional {
            true => code! { .optional() },
            _ => None,
        };

        let build_variadic = match variadic {
            true => code! { .variadic() },
            _ => None,
        };

        let build_conflicts = match conflicts.is_empty() {
            false => code! { .conflicts(&[#(#conflicts),*]) },
            _ => None,
        };

        declare_non_options.push(code! {
            let mut #variable = __private::non_option(#name)
                #build_value_or_command
                #build_optional
                #build_variadic
                #build_conflicts
                ;
        });

        declare_fields.push(code! {
            pub #field: #field_ty,
        });

        assign_fields.push(match kind {
            IrNonOptionKind::Command(_) => code! {
                #field: #variable.into(&bin_name)?,
            },
            _ => code! {
                #field: #variable.into(),
            },
        });

        setters_non_options.push(code! { #variable.as_setter(), });
    }

    let ident = &ir.ident;
    let version = version(&ir);
    let help = help(&ir);
    let command = command(&ir);

    Ok(quote! {
        #[allow(unused)]
        #[derive(Debug)]
        pub struct #ident {
            #(#declare_fields)*
        }

        #command

        #[allow(unused)]
        #[automatically_derived]
        impl ::immargs::FromArgs for #ident {
            fn from_args(mut args: ::immargs::Args) -> ::immargs::Result<Self> {
                use ::immargs::__private;
                #version
                let bin_name = __private::bin_name(&mut args);
                #help
                #(#declare_options)*
                #(#declare_non_options)*
                __private::parse(args, &mut [#(#setters_options)*], &mut [#(#setters_non_options)*])?;
                Ok(Self {
                    #(#assign_fields)*
                })
            }
        }

        #[allow(unused)]
        #[automatically_derived]
        impl #ident {
            pub fn try_from<T: IntoIterator<Item: Into<String>>>(args: T) -> ::immargs::Result<Self> {
                ::immargs::__private::try_from(args)
            }

            pub fn try_from_env() -> ::immargs::Result<Self> {
                ::immargs::__private::try_from_env()
            }

            pub fn from<T: IntoIterator<Item: Into<String>>>(args: T) -> Self {
                ::immargs::__private::from(args)
            }

            pub fn from_env() -> Self {
                ::immargs::__private::from_env()
            }
        }
    })
}

fn command(ir: &Ir) -> Option<TokenStream> {
    fn variant(s: &str) -> Ident {
        let mut next_to_uppercase = true;
        let normalized = s
            .chars()
            .filter_map(|c| match (c, next_to_uppercase) {
                ('-', _) => {
                    next_to_uppercase = true;
                    None
                }
                (_, true) => {
                    next_to_uppercase = false;
                    c.to_uppercase().next()
                }
                (_, false) => {
                    next_to_uppercase = false;
                    c.to_lowercase().next()
                }
            })
            .collect::<String>();
        format_ident!("{normalized}")
    }

    let (ty, commands) = ir.non_options.iter().find_map(|arg| match &arg.kind {
        IrNonOptionKind::Command(commands) => Some(commands),
        _ => None,
    })?;

    let variants = commands
        .iter()
        .map(|command| {
            let first = &command.names.first().unwrap();
            let variant = variant(first);
            quote! { #variant(::immargs::Args), }
        })
        .collect::<Vec<_>>();

    let match_normalize = commands
        .iter()
        .map(|command| {
            let all = &command.names;
            let first = &command.names.first().unwrap();
            quote! { #(#all)|* => Ok(#first), }
        })
        .collect::<Vec<_>>();

    let match_from = commands
        .iter()
        .map(|command| {
            let first = &command.names.first().unwrap();
            let variant = variant(first);
            quote! { #first => Ok(Self::#variant(args)), }
        })
        .collect::<Vec<_>>();

    let match_into_str = commands
        .iter()
        .map(|command| {
            let first = &command.names.first().unwrap();
            let variant = variant(first);
            quote! { Self::#variant(args) => (#first, args), }
        })
        .collect::<Vec<_>>();

    code! {
        #[allow(unused)]
        #[derive(Debug)]
        pub enum #ty {
            #(#variants)*
        }
        #[allow(unused)]
        #[automatically_derived]
        impl ::immargs::__private::Command for #ty {
            fn normalize(command: &str) -> ::immargs::Result<&'static str> {
                match command {
                    #(#match_normalize)*
                    _ => Err(::immargs::Error::InvalidCommand { arg: command.to_string() }),
                }
            }

            fn from(command: &str, args: ::immargs::Args) -> ::immargs::Result<Self> {
                match command {
                    #(#match_from)*
                    _ => Err(::immargs::Error::InvalidCommand { arg: command.to_string() }),
                }
            }
        }
        #[allow(unused)]
        #[automatically_derived]
        impl #ty {
            pub fn into_str(self) -> (&'static str, ::immargs::Args) {
                match self {
                    #(#match_into_str)*
                }
            }
        }
    }
}

fn version(ir: &Ir) -> Option<TokenStream> {
    ir.options
        .iter()
        .find(|option| matches!(option.kind, IrOptionKind::Version))?;

    code! {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
    }
}

fn help(ir: &Ir) -> Option<TokenStream> {
    fn help_options(ir: &Ir) -> Vec<(&String, Option<&String>)> {
        ir.options
            .iter()
            .map(|arg| (&arg.usage, arg.help.as_ref()))
            .collect::<Vec<_>>()
    }

    fn help_non_options(ir: &Ir) -> Vec<(&String, Option<&String>)> {
        ir.non_options
            .iter()
            .filter_map(|arg| arg.help.as_ref().map(|help| (&arg.usage, Some(help))))
            .collect::<Vec<_>>()
    }

    fn help_commands(ir: &Ir) -> Vec<(&String, Option<&String>)> {
        ir.non_options
            .last()
            .filter(|arg| matches!(arg.kind, IrNonOptionKind::Command(_)))
            .map(|arg| {
                let IrNonOptionKind::Command((_, commands)) = &arg.kind else {
                    unreachable!();
                };
                commands
                    .iter()
                    .map(|command| (&command.usage, command.help.as_ref()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    fn help0_width(help: &Vec<(&String, Option<&String>)>) -> usize {
        help.iter()
            .map(|help| help.0.len())
            .max()
            .unwrap_or_default()
    }

    fn help_usage(ir: &Ir) -> (String, String) {
        let usage0 = String::from("usage: ");
        let mut usage1 = String::new();

        if !ir.options.is_empty() {
            usage1.push_str(" [options]");
        }

        for arg in &ir.non_options {
            usage1.push(' ');
            usage1.push_str(&arg.usage);
            if matches!(arg.kind, IrNonOptionKind::Command(_)) {
                usage1.push_str(" [...]");
            }
        }

        usage1.push_str("\n\n");
        (usage0, usage1)
    }

    fn help_section(
        title: &str,
        width: usize,
        help: Vec<(&String, Option<&String>)>,
    ) -> Vec<String> {
        let mut section = vec![];

        if !help.is_empty() {
            section.push(format!("{title}:\n"));

            for (help0, help1) in help {
                match help1 {
                    Some(help1) => section.push(format!("   {help0:<width$}     {help1}\n")),
                    None => section.push(format!("   {help0}\n")),
                }
            }

            section.last_mut().unwrap().push('\n');
        }

        section
    }

    ir.options
        .iter()
        .find(|option| matches!(option.kind, IrOptionKind::Help))?;

    let options = help_options(ir);
    let non_options = help_non_options(ir);
    let commands = help_commands(ir);

    let width = help0_width(&options)
        .max(help0_width(&non_options))
        .max(help0_width(&commands));

    let (usage0, usage1) = help_usage(ir);
    let options = help_section("options", width, options);
    let non_options = help_section("arguments", width, non_options);
    let commands = help_section("commands", width, commands);
    let sections = [options, non_options, commands].concat();

    code! {
        let help = [
            #usage0,
            &bin_name,
            #usage1,
            #(#sections),*
        ];
    }
}
