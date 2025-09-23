#![doc(hidden)]

use proc_macro2::Span;
use syn::Ident;
use syn::LitStr;
use syn::Result;
use syn::TypePath;
use syn::braced;
use syn::bracketed;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Brace;
use syn::token::Bracket;
use syn::token::Comma;
use syn::token::DotDotDot;
use syn::token::Gt;
use syn::token::Lt;
use syn::token::Minus;
use syn::token::Not;
use syn::token::Question;

pub struct Ast {
    pub ident: Option<Ident>,
    pub arguments: AstArguments,
}

pub struct AstArguments(pub Vec<AstArgument>);

pub enum AstArgument {
    Option(AstOption),
    NonOption(AstNonOption),
}

pub struct AstOption {
    pub span: Span,
    pub shorts: AstOptionShorts,
    pub longs: AstOptionLongs,
    pub variadic: AstVariadic,
    pub value: AstOptionValue,
    pub conflicts: AstConflicts,
    pub help: AstHelp,
}

pub struct AstNonOption {
    pub span: Span,
    pub optional: bool,
    pub ident: Ident,
    pub variadic: AstVariadic,
    pub ty: TypePath,
    pub conflicts: AstConflicts,
    pub help: AstHelp,
    pub commands: AstCommands,
}

pub struct AstOptionShorts(pub Vec<AstOptionShort>);

pub struct AstOptionShort(pub Ident);

pub struct AstOptionLongs(pub Vec<AstOptionLong>);

pub struct AstOptionLong(pub Ident);

pub struct AstOptionValue(pub Option<(Ident, TypePath)>);

pub struct AstVariadic(pub Option<DotDotDot>);

pub struct AstConflicts(pub Vec<AstConflict>);

pub struct AstConflict(pub char, pub Span, pub Option<Ident>);

pub struct AstHelp(pub Option<LitStr>);

pub struct AstCommands(pub Option<Vec<AstCommand>>);

pub struct AstCommand {
    pub idents: Vec<Ident>,
    pub help: AstHelp,
}

impl Parse for Ast {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            input.parse::<Comma>()?;
            Some(ident)
        } else {
            None
        };

        let arguments = input.parse::<AstArguments>()?;
        Ok(Self { ident, arguments })
    }
}

impl Parse for AstArguments {
    fn parse(input: ParseStream) -> Result<Self> {
        let arguments = Punctuated::<AstArgument, Comma>::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(Self(arguments))
    }
}

impl Parse for AstArgument {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Minus) {
            input.parse().map(AstArgument::Option)
        } else if input.peek(Lt) || input.peek(Bracket) {
            input.parse().map(AstArgument::NonOption)
        } else {
            Err(input.error("expected '-', '[', or '<'"))
        }
    }
}

impl Parse for AstOption {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let shorts = input.parse::<AstOptionShorts>()?;
        let longs = input.parse::<AstOptionLongs>()?;
        let variadic = input.parse::<AstVariadic>()?;
        let value = input.parse::<AstOptionValue>()?;
        let conflicts = input.parse::<AstConflicts>()?;
        let help = input.parse::<AstHelp>()?;

        Ok(AstOption {
            span,
            shorts,
            longs,
            variadic,
            value,
            conflicts,
            help,
        })
    }
}

impl Parse for AstNonOption {
    fn parse(input: ParseStream) -> Result<Self> {
        fn value(input: ParseStream) -> Result<(Ident, AstVariadic)> {
            input.parse::<Lt>()?;
            let ident = input.parse::<Ident>()?;
            input.parse::<Gt>()?;
            let variadic = input.parse::<AstVariadic>()?;
            Ok((ident, variadic))
        }

        let span = input.span();
        let (optional, ident, variadic) = if input.peek(Bracket) {
            let content;
            bracketed!(content in input);
            let (ident, variadic) = value(&content)?;
            (true, ident, variadic)
        } else {
            let (ident, variadic) = value(input)?;
            (false, ident, variadic)
        };

        let ty = input.parse::<TypePath>()?;
        let conflicts = input.parse::<AstConflicts>()?;
        let help = input.parse::<AstHelp>()?;
        let commands = input.parse::<AstCommands>()?;

        Ok(Self {
            span,
            optional,
            ident,
            variadic,
            ty,
            conflicts,
            help,
            commands,
        })
    }
}

impl Parse for AstOptionShorts {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut shorts = vec![];

        while input.peek(Minus) && !input.peek2(Minus) {
            shorts.push(input.parse::<AstOptionShort>()?);
        }

        Ok(Self(shorts))
    }
}

impl Parse for AstOptionShort {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Minus>()?;
        let short = input.parse::<Ident>()?;
        Ok(Self(short))
    }
}

impl Parse for AstOptionLongs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut longs = vec![];

        while input.peek(Minus) && input.peek2(Minus) {
            longs.push(input.parse::<AstOptionLong>()?);
        }

        Ok(Self(longs))
    }
}

impl Parse for AstOptionLong {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Minus>()?;
        input.parse::<Minus>()?;
        let long = input.parse::<Ident>()?;
        Ok(Self(long))
    }
}

impl Parse for AstOptionValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let value = if input.peek(Lt) {
            input.parse::<Lt>()?;
            let ident = input.parse::<Ident>()?;
            input.parse::<Gt>()?;
            let type_path = input.parse::<TypePath>()?;
            Some((ident, type_path))
        } else {
            None
        };

        Ok(Self(value))
    }
}

impl Parse for AstVariadic {
    fn parse(input: ParseStream) -> Result<Self> {
        let dotdotdot = if input.peek(DotDotDot) {
            Some(input.parse::<DotDotDot>()?)
        } else {
            None
        };

        Ok(Self(dotdotdot))
    }
}

impl Parse for AstConflicts {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut conflicts = vec![];

        while input.peek(Question) || input.peek(Not) {
            conflicts.push(input.parse::<AstConflict>()?);
        }

        Ok(Self(conflicts))
    }
}

impl Parse for AstConflict {
    fn parse(input: ParseStream) -> Result<Self> {
        let (kind, span) = if input.peek(Question) {
            let question = input.parse::<Question>()?;
            ('?', question.span())
        } else {
            let not = input.parse::<Not>()?;
            ('!', not.span())
        };

        if input.peek(Ident) {
            let id = input.parse::<Ident>()?;
            Ok(Self(kind, id.span(), Some(id)))
        } else {
            Ok(Self(kind, span, None))
        }
    }
}

impl Parse for AstHelp {
    fn parse(input: ParseStream) -> Result<Self> {
        let help = if input.peek(LitStr) {
            Some(input.parse::<LitStr>()?)
        } else {
            None
        };

        Ok(Self(help))
    }
}

impl Parse for AstCommand {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut idents = vec![];

        while input.peek(Ident) {
            idents.push(input.parse::<Ident>()?);
        }

        let help = input.parse::<AstHelp>()?;

        Ok(Self { idents, help })
    }
}

impl Parse for AstCommands {
    fn parse(input: ParseStream) -> Result<Self> {
        let commands = if input.peek(Brace) {
            let content;
            braced!(content in input);
            let commands = Punctuated::<AstCommand, Comma>::parse_terminated(&content)?
                .into_iter()
                .collect();
            Some(commands)
        } else {
            None
        };

        Ok(Self(commands))
    }
}
