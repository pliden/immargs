//! Please see documentation for [`immargs`](../immargs/index.html) crate.
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
/// Please see [crate-level documentation](index.html) for additional information.
#[proc_macro]
pub fn immargs(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as Ast);
    let ir = catch_error!(lower(ast));
    let code = catch_error!(emit(ir));
    code.into()
}
