#![doc(hidden)]

use crate::Args;
use crate::Error;
use crate::FromArgs;
use crate::Result;
use std::io::Write;
use std::io::stdout;
use std::path::PathBuf;
use std::process::exit;

#[inline]
pub fn try_from<T: FromArgs, I: IntoIterator<Item: Into<String>>>(args: I) -> Result<T> {
    T::from_args(Args::from(args))
}

#[inline]
pub fn try_from_env<T: FromArgs>() -> Result<T> {
    T::from_args(Args::from_env())
}

#[inline]
pub fn try_from_args<T: FromArgs>(args: Args) -> Result<T> {
    T::from_args(args)
}

#[inline]
pub fn from<T: FromArgs, I: IntoIterator<Item: Into<String>>>(args: I) -> T {
    exit_on_error(try_from(args))
}

#[inline]
pub fn from_env<T: FromArgs>() -> T {
    exit_on_error(try_from_env())
}

#[inline]
pub fn from_args<T: FromArgs>(args: Args) -> T {
    exit_on_error(try_from_args(args))
}

#[inline]
pub fn bin_name(args: &mut Args) -> String {
    PathBuf::from(args.pop().unwrap_or_default())
        .file_name()
        .map(|bin_name| bin_name.to_string_lossy().into_owned())
        .unwrap_or(String::from("<program>"))
}

#[inline]
fn exit_on_error<T>(args: Result<T>) -> T {
    match args {
        Ok(args) => args,
        Err(error) => {
            let (prefix, postfix, exit_code) = match error {
                Error::Version { message: _ } => ("", "\n", 0),
                Error::Help { message: _ } => ("", "", 0),
                _ => ("error: ", "\n", 1),
            };
            let _ = write!(stdout(), "{prefix}{error}{postfix}");
            exit(exit_code);
        }
    }
}
