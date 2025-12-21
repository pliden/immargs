#![doc(hidden)]

use crate::Args;
use crate::Error;
use crate::Result;
use std::io::Write;
use std::io::stdout;
use std::path::PathBuf;
use std::process::exit;

#[inline]
pub fn bin_name(args: &mut Args) -> String {
    PathBuf::from(args.pop().unwrap_or_default())
        .file_name()
        .map(|bin_name| bin_name.to_string_lossy().into_owned())
        .unwrap_or(String::from("<program>"))
}

#[inline]
pub fn try_from<T>(args: Result<T>) -> Result<Option<T>> {
    match args {
        Err(Error::Help { message }) => {
            let _ = writeln!(stdout(), "{message}");
            Ok(None)
        }
        Err(Error::Version { message }) => {
            let _ = writeln!(stdout(), "{message}");
            Ok(None)
        }
        _ => args.map(|args| Some(args)),
    }
}

#[inline]
pub fn from<T>(args: Result<T>) -> T {
    match try_from(args) {
        Ok(Some(args)) => args,
        Ok(None) => exit(0),
        Err(error) => {
            let _ = writeln!(stdout(), "error: {error}");
            exit(1);
        }
    }
}
