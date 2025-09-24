//! Please see documentation for [`immargs`](https://docs.rs/immargs/) crate.
#![doc(hidden)]

use ast::Ast;
use code::emit;
use ir::lower;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod ast;
mod code;
mod ir;

macro_rules! catch_error {
    ($expr:expr) => {
        match $expr {
            Ok(ok) => ok,
            Err(err) => return err.to_compile_error().into(),
        }
    };
}

/// Macro for declaring command line arguments.
///
/// Please see [crate-level documentation](index.html) for additional information
/// and examples on how to use this macro.
///
/// ## Syntax Specification
///
/// `immargs! {`
///     \[ ___StructName___ `,` \]
///     \[ ___Option___ `,` \]*
///     \[ ___NonOption___ `,` \]*
/// `}`
///
/// ___Option___ := \[ `-` ___Short___ \]*
///                 \[`--` ___Long___ \]*
///                 \[ `...` \]
///                 \[ `<` ___Value___ `>` ___Type___ \]
///                 \[ \( `!` | `?` \) \[ ___ConflictId___ \] \]*
///                 \[ ___Help___ \]
///                 `,`
///
/// ___NonOption___ := \( ___RequiredNonOption___ | ___OptionalNonOption___  \)*
///
/// ___RequiredNonOption___ := `<` ___Name___ `>`
///                            \[ `...` \]
///                            ___Type___
///                            \[ \( `!` | `?` \) \[ ___ConflictId___ \] \]*
///                            \[ ___Help___ \]
///                            \[ ___Commands___ \]
///                            `,`
///
/// ___OptionalNonOption___ := `[<` ___Name___ `>` \[ `...` \] `]`
///                            ___Type___
///                            \[ \( `!` | `?` \) \[ ___ConflictId___ \] \]*
///                            \[ ___Help___ \]
///                            \[ ___Commands___ \]
///                            `,`
///
/// ___Commands___ := `{` \[ ___Command___ `,` \]* `}`
///
/// ___Command___ := ___Name___ \[ ___Alias___ \]* \[ ___Help___ \]
///
/// ___StructName___ /
/// ___Short___ /
/// ___Long___ /
/// ___Value___ /
/// ___Name___ /
/// ___Alias___ /
/// ___ConflictId___ := A Rust [non-keyword identifier](https://doc.rust-lang.org/reference/identifiers.html)
///
/// ___Type___ := A Rust type that implements [`FromStr`](std::str::FromStr) + [`Debug`](std::fmt::Debug)
///
/// ___Help___ := A Rust [string literal](https://doc.rust-lang.org/reference/tokens.html#r-lex.token.literal.str)
///
/// ## Examples
///
/// Options:
///
/// ```no_rust
/// -f,                                        // Short-option
/// --foo,                                     // Long-option
/// -f --foo,                                  // Short-option + long-option
/// -f -F --foo,                               // Multiple short-options + long-option
///
/// -f <bar> u64,                              // With u64 value named "bar"
/// --foo <bar> u64,                           // With u64 value named "bar"
/// -f --foo <bar> String,                     // With String value named "bar"
/// -f -F --foo <bar> Ipv4Addr,                // With Ipv4Addr value named "bar"
///
/// -f                          "Help text",   // With help text
/// --foo                       "Help text",   // ...
/// -f --foo                    "Help text",   // ...
/// -f --foo <bar> u64          "Help text",   // ...
///
/// -f                       !  "Help text",   // With default conflict-id
/// --foo                    !  "Help text",   // ...
/// -f --foo                 ?  "Help text",   // ...
/// -f --foo <bar> u64       ?  "Help text",   // ...
/// -f --foo <bar> String !A !B "Help text",   // With conflict-ids "A" and "B"
///
/// -f...                       "Help text",   // Variadic (repeatable) option
/// --foo...                    "Help text",   // ...
/// -f --foo...                 "Help text",   // ...
/// -f --foo... <bar> u64       "Help text",   // ...
/// -f --foo... <bar> String    "Help text",   // ...
/// ```
///
/// Non-options:
///
/// ```no_rust
/// <foo> u64,                                 // Required argument named "foo" of type u64
/// <foo> String,                              // Required argument named "foo" of type String
/// <foo>... String,                           // Required variadic argument named "foo" of type String
/// <foo>... String "Help text",               // With help text
/// <foo>... String ! "Help text",             // With default conflict-id
/// <foo>... String !A !B "Help text",         // With conflict-ids "A" and "B"
///
/// [<foo>] u64,                               // Optional argument name "foo" of type u64
/// [<foo>] String,                            // Optional argument name "foo" of type String
/// [<foo>...] String,                         // Optional variadic argument name "foo" of type String
/// [<foo>...] String "Help text",             // With help text
/// [<foo>...] String ! "Help text",           // With default conflict-id
/// [<foo>...] String !A !B "Help text",       // With conflict-ids "A" and "B"
///
/// <command> Command {                        // Required command argument
///     add,                                   // Command "add"
///     remove rm,                             // Command "remove" with alias "rm"
///     list ls l,                             // Command "list" with alieses "ls" and "l"
/// }
///
/// [<command>] Command {                      // Optional command
///     add,
///     remove rm,
///     list ls l,
/// }
///
/// <command> Command "Help text" {            // With help texts
///     add         "Help text for add",
///     remove rm   "Help text for remove",
///     list ls l   "Help text for list",
/// }
///
/// <command> Command ! "Help text" {          // With default conflict-id
///     add         "Help text for add",
///     remove rm   "Help text for remove",
///     list ls l   "Help text for list",
/// }
///
/// <command> Command !A !B "Help text" {      // With conflict-ids "A" and "B"
///     add         "Help text for add",
///     remove rm   "Help text for remove",
///     list ls l   "Help text for list",
/// }
/// ```
#[proc_macro]
pub fn immargs(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as Ast);
    let ir = catch_error!(lower(ast));
    let code = catch_error!(emit(ir));
    code.into()
}
