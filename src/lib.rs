//! # Immediate Arguments
//!
//! _No-hassle, on-the-spot, command line argument parser_
//!
//! Highlights:
//!
//! * Straightforward declaration of arguments with proc-macro.
//! * Supports POSIX/GNU argument syntax conventions.
//! * Supports arguments of any type that implements [`FromStr`](core::str::FromStr) + [`Debug`](core::fmt::Debug).
//! * Supports (sub)commands, with aliases.
//! * Supports declaration of conflicting arguments.
//! * Supports automatic `--version` and `--help` handling, with possibility to opt-out.
//! * Tested on Linux, macOS and Windows.
//! * No run-time dependencies.
//!
//! # Basic Example
//!
//! This exanple is using [`immargs_from_env!`] for on-the-spot declaration and parsing of
//! command line arguments.
//!
//! ```no_run
//! use immargs::immargs_from_env;
//!
//! let args = immargs_from_env! {
//!     --force               "overwrite destination",
//!     -l --log <level> u8   "set log level",
//!     -h --help             "print help message",
//!     <src>... String       "source(s)",
//!     <dest> String         "destination",
//! };
//!
//! // Assuming this program was executed with "myprog -l 3 Src0 Src1 Dest"
//! assert!(!args.force);
//! assert!(args.log == Some(3));
//! assert!(args.src.len() == 2);
//! assert!(args.src[0] == "Src0");
//! assert!(args.src[1] == "Src1");
//! assert!(args.dest == "Dest");
//! ```
//!
//! [`immargs_from_env!`] returns a `struct` with fields derived from to the arguments
//! specification. The fields are populated with the corresponding values from the command
//! line. For the example above, the returned `struct` looks like this:
//! ```
//! pub struct ImmArgs {
//!     pub force: bool,
//!     pub log: Option<u8>,
//!     pub src: Vec<String>,
//!     pub dest: String,
//! }
//! ```
//!
//! A help message will also be derived from the arguments specification, and is printed if the
//! `-h` or `--help` option is used. For the above example, the help message looks like this:
//!
//! ```no_rust
//! usage: myprog [options] <src>... <dest>
//!
//! options:
//!    --force               overwrite destination
//!    -l, --log <level>     set log level
//!    -h, --help            print help message
//!
//! arguments:
//!    <src>...              source(s)
//!    <dest>                desination
//!
//! ```
//!
//! # Advanced Example
//!
//! This example is using [`immargs!`] to declare command line arguments. The main program
//! takes a command as its last argument, and each (sub)command use [`immargs!`] do declare
//! the arguments they each accept. Any arguments following the command will be returned as
//! an [`Args`] that is part of the command enum, which is converted into the (sub)command
//! arguments `struct` using [`into()`](Args::into).
//!
//! ```no_run
//! use immargs::immargs;
//!
//! // The last argument is a command, with available commands (and their aliases) enclosed
//! // by braces. An `enum`, with variants matching the commands, will be generated.
//! immargs! {
//!     MainArgs,                                         // Argument struct will be called "MainArgs"
//!     -v --verbose           "enable verbose logging",  // An option
//!     --version              "print version",           // --version enables automatic version printing
//!     -h --help              "print help message",      // --help enables automatic help printing
//!     <command> Command      "the command to run" {     // Command enum will be named "Command"
//!         add                "add file(s)",             // "add" has no aliases
//!         remove rm          "remove file(s)",          // "rm" is an alias for "remove"
//!         commit co c        "commit changes",          // "co" and "c" are aliases for "commit"
//!     }
//! }
//!
//! // The "?" indicates that these are conflicting arguments, i.e. they can't be
//! // used at the same time, but one of them must be present on the command line.
//! immargs! {
//!     AddArgs,
//!     -a --all            ?  "add all files",           // Conflicts with [<file>...]
//!     --force                "overwrite destination",
//!     -h --help              "print help message",
//!     [<file>...] String  ?  "file(s) to add",          // Conflicts with -a, --all
//! }
//!
//! immargs! {
//!     RemoveArgs,
//!     -r --recursive         "recursively remove files",
//!     -h --help              "print help message",
//!     <file>... String       "file(s) to remove",       // "..." means it's a variadic argument
//! }
//!
//! immargs!(
//!     CommitArgs,
//!     -a --amend             "amend latest commit",
//!     -h --help              "print help message",
//!     [<message>] String     "commit message",          // "[ ]" means it's an optional argument
//! );
//!
//! fn main() {
//!     let main_args = MainArgs::from_env();
//!     let verbose = main_args.verbose;
//!
//!     match main_args.command {
//!         Command::Add(args) => add(verbose, args.into()),
//!         Command::Remove(args) => remove(verbose, args.into()),
//!         Command::Commit(args) => commit(verbose, args.into()),
//!     }
//! }
//!
//! fn add(verbose: bool, args: AddArgs) {
//!     // ...
//! }
//!
//! fn remove(verbose: bool, args: RemoveArgs) {
//!     // ...
//! }
//!
//! fn commit(verbose: bool, args: CommitArgs) {
//!     // ...
//! }
//! ```
//!
//! # Terminology
//!
//! [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap12.html
//! [GNU]: https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html
//!
//! Terms and definitions used by `immargs` (mostly derived from [POSIX] and [GNU] definitions):
//!
//! * __Arguments__ - The umbrella term for all kinds of strings that appear on the command line.
//!   _Arguments_ are further divided into _options_ and _non-options_.
//!
//! * __Options__ - The subset of _arguments_ that start with `-` or `--`, e.g. `-v` or
//!   `--verbose`. The order in which _options_ appear on the command line carries no meaning,
//!   i.e. `-a -b` has the same meaning as `-b -a`. _Options_ are, as the name implies, always
//!   optional and never required. _Options_ are further divided into _short_ and _long_ options.
//!
//! * __Short options__ - The subset of _options_ that start with `-` followed by a single
//!   character, e.g. `-v`.
//!
//! * __Long options__ - The subset of _options_ that start with `--` followed by two or more
//!   characters, e.g. `--verbose`.
//!
//! * __Value__ - An additional piece of information associated with an _option_, e.g.
//!   `--speed 100`, where `100` is the _value_. Value-less _options_ implicitly hold a binary
//!   value (`true` or `false`) through their presence or absence on the command line.
//!
//! * __Non-options__ - The subset of _arguments_ that don't start with `-` or `--`, e.g.
//!   `commit` or `file.txt`. The order in which _non-options_ appear on the command line
//!   carries meaning, i.e. `commit file.txt` doesn't have the same meaning as `file.txt commit`.
//!   _Non-optons_ are further divided into _required_ and _optional_ non-options.
//!
//! * __Required non-option__ - A _non-option_ that must be present on the command line.
//!   In text, _required non-options_ are represented by enclosing `<` `>`, e.g. `<file>`.
//!
//! * __Optional non-option__ - A _non-options_ that doesn't have to be present on the command
//!   line. In text, _optional non-options_ are represented by enclosing `[<` `>]`, e.g. `[<file>]`.
//!
//! * __Variadic option__ - An _option_ that is materialized from multiple separate instances
//!   of the option, as opposed to the last instance taking precedence.
//!   E.g. `--file hello.txt --file world.txt` results in the option _file_ holding two values,
//!   `hello.txt` and `world.txt`. A value-less _option_ can also be _variadic_, in which case
//!   the value held by the option is an unsigned integer representing the number of times the
//!   option appeared on the command line. E.g. `--verbose --verbose --verbose` results in the
//!   option _verbose_ holding the value `3`. In text, _variadic options_ are represented by
//!   trailing `...`, e.g. `--file... <path>` or `--verbose...`.
//!
//! * __Variadic non-option__ - A _non-option_ that is materialized from multiple separate
//!   command line arguments of the same type. In text, _variadic non-options_ are represented
//!   by trailing `...`, e.g. `<file>...` or `[<file>...]`.
//!
//! # Command Line Argument Syntax
//!
//! The following [POSIX]/[GNU] argument syntax conventions are supported:
//!
//! * Short option, `-` followed by a single character, e.g. `-f`.
//! * Long option, `--` followed by two or more characters, e.g. `--foo`.
//! * Short/Long option with separate value, e.g. `-f 100` or `--foo 100`.
//! * Short/Long option with attached value delimited by `=`, e.g. `-f=100` or `--foo=100`.
//! * Short option with attached value without delimiter, e.g. `-f100`.
//! * Combined short options, e.g. `-abc` is equivalent to `-a -b -c`.
//! * Short/Long options may appear in any order, but must come before any non-option arguments.
//! * Short/Long options may appear multiple times, the last appearance takes precedence unless
//!   it's a _variadic_ (repeatable) option, where the number of times the option appears has
//!   meaning, e.g. `-vvv` where each `-v` increases the verbosity level.
//! * The order of non-option arguments carries meaning.
//! * A standalone `-` argument is treated as a non-option argument.
//! * A standalone `--` argument marks the end of options. Any following arguments are treated
//!   as non-option arguments.
//!
//! # The Returned `struct`
//!
//! #### Field Names
//!
//! The field names of the `struct` generated by [`immargs!`] are derived as follows.
//!
//! | Argument Type | Field Name | Example |
//! | - | - | - |
//! | Option | Field name is direved from the first long-option (or the first short-option if no long-option exists) | `--foo` uses field name `foo` |
//! | Non-Option | Field name is derived from the non-option name | `<bar> T` uses field name `bar` |
//!
//! Note that the specified option and non-option names must be valid Rust identifiers, as they
//! will become the names of the `struct` fields. However, the names visible to the user of the
//! program will be transformed as follows, to allow use of names that aren't valid Rust `struct`
//! field names (keywords, words starting with a number, etc).
//!
//! * Any starting and trailing `_` will be stripped.
//! * Any other `_` will be replaced by `-`.
//! * Letters will be converted to lower case (does not apply to short-options).
//!
//! Examples:
//!
//! | Specified Argument | `struct` Field Name | User Visibale Name |
//! | - | - | - |
//! | `-_1` | _1 | `-1` |
//! | `--move_` | move_ | `--move` |
//! | `--_1_to_1` | _1_to_1 | `--1-to-1` |
//! | `--Report_Error` | `Report_Error` | `--report-error` |
//! | `--log <Log_Level> u8` | `log` | `--log <log-level>` |
//! | `<number_of_items>` | `number_or_items` | `<number-of-items>` |
//! | `[_4th]` | `_4th` | `[4th]` |
//!
//! #### Field Types
//!
//! The field types of the `struct` generated by [`immargs!`] are derived as follows.
//!
//! | Argument Type | Example | Field Type |
//! | - | - | - |
//! | Option | `--foo` | bool |
//! | Option with Value | `--foo <bar> T` | `Option<T>` |
//! | Variadic Option | `--foo...` | `usize` |
//! | Variadic Option with Value | `--foo... <bar> T` | `Vec<T>` |
//! | Required Non-option | `<foo> T` | `T` |
//! | Optional Non-option | `[<foo>] T` | `Option<T>` |
//! | Required Variadic Non-option | `<foo>... T` | `Vec<T>` (with lenth > 0) |
//! | Optional Variadic Non-option | `[<foo>...] T` | `Vec<T>` (with lenth >= 0) |
//! | Required Command Non-option | `<foo> T` | `T(`[`Args`]`)` |
//! | Optional Command Non-option | `[<foo>] T` | `Option<T(`[`Args`]`)>` |
//!
//! #### Methods
//!
//! The following methods are available on arguments `struct`s generated by [`immargs!`].
//!
//! | Method | Return Type |
//! | - | - |
//! | `from_env()` | `Self` |
//! | `from<T: IntoIterator<Item: Into<String>>>(args: T)` | `Self` |
//! | `try_from_env()` | `Result<Self>` |
//! | `try_from<T: IntoIterator<Item: Into<String>>>(args: T)` | `Result<Self>` |
//!
//! Most applications would want to use `from_env()`, which uses arguments provided by
//! [`std::env::args_os()`] and on failure prints an error message and terminates the
//! program with an appropriate exit code.
//!
//! The other methods exist to allow applications to opt-out of the default behaviours
//! provided by `from_env()`, such as explicltly providing the command line arguments to
//! parse, or to implement custom error and help handling.
//!
//! # Conflicting Arguments
//!
//! An argument can be declared to be in conflict with one or more other arguments. This is
//! useful when two or more arguments are incompatible or otherwise mutually exclusive. A
//! conflict is declared using an `!` or `?` optionally followed by a _conflict-id_. The
//! _conflict-id_ is an identitier used if there are more than one group of conflicting
//! options. `!` (plain conflict) means that zero or one of the options in the group is
//! allowed to be present on the command line, while `?` (choice) means that exactly one of
//! the options in the group is must be present on the command line.
//!
//! Example with one group of conflicting arguments, i.e. without an explicit _conflict-id_,
//! where exacly one of them must be present on the command line:
//!
//! ```
//! use immargs::immargs;
//!
//! immargs! {
//!     --verbose,                   // Doesn't conflict with any other argument
//!     --all                 ?,     // Conflicts with [names...]
//!     [<names>...] String   ?,     // Conflicts with --all
//! }
//! ```
//!
//! Example with two groups (`B_C` and `C_D_E`) of conflicting arguments, where one argument is
//! part of both groups. Exactly one of the options in group `B_C` must be present, and zero or
//! one of the options in group `C_D_E` is allowed to be present on the command line.
//!
//! ```
//! use immargs::immargs;
//!
//! immargs! {
//!     --feature_a,                 // Doesn't conflict with any other argument
//!     --feature_b   ?B_C,          // Conflicts with --feature-c
//!     --feature_c   ?B_C !C_D_E,   // Conflicts with --feature-b, --feature-d and --feature-e
//!     --feature_d   !C_D_E,        // Conflicts with --feature-c and --feature-e
//!     --feature_e   !C_D_E,        // Conflicts with --feature-c and --feature-d
//! }
//! ```
//!
//! # Help and Version
//!
//! Options with long-option names `--help` and `--version` are special. These options are
//! intercepted during arguments parsing and will not be visible, or have corresponding fields,
//! in the arguments `struct` generated by [`immargs!`]. When parsing the command line using
//! `from()` or `from_env()`, these options will cause a version or help message to be displayed
//! and the application will be terminated. When parsing the command line using `try_from()` or
//! `try_from_env()` these options will instead generate a [`Help`](Error::Help) or
//! [`Version`](Error::Version) error, which the application can react to, e.g. if the application
//! wants to display custom version or help messages.
//!
//! # Unicode
//!
//! Non-unicode command line arguments will be converted to unicode using
//! [`to_string_lossy()`](std::ffi::OsStr::to_string_lossy) before they are parsed.
//!
//! # `immargs!()` Syntax
//!
//! ## Specification
//!
//! `immargs! {`
//!     \[ ___StructName___ `,` \]
//!     \[ ___Option___ `,` \]*
//!     \[ ___NonOption___ `,` \]*
//! `}`
//!
//! ___Option___ := \[ `-` ___Short___ \]*
//!                 \[`--` ___Long___ \]*
//!                 \[ `...` \]
//!                 \[ `<` ___Value___ `>` ___Type___ \]
//!                 \[ `!` \[ ___ConflictId___ \] \]*
//!                 \[ `?` \[ ___ConflictId___ \] \]*
//!                 \[ ___Help___ \]
//!                 `,`
//!
//! ___NonOption___ := \( ___RequiredNonOption___ | ___OptionalNonOption___  \)*
//!
//! ___RequiredNonOption___ := `<` ___Name___ `>`
//!                            \[ `...` \]
//!                            ___Type___
//!                            \[ `!` \[ ___ConflictId___ \] \]*
//!                            \[ `?` \[ ___ConflictId___ \] \]*
//!                            \[ ___Help___ \]
//!                            \[ ___Commands___ \]
//!                            `,`
//!
//! ___OptionalNonOption___ := `[<` ___Name___ `>` \[ `...` \] `]`
//!                            ___Type___
//!                            \[ `!` \[ ___ConflictId___ \] \]*
//!                            \[ `?` \[ ___ConflictId___ \] \]*
//!                            \[ ___Help___ \]
//!                            \[ ___Commands___ \]
//!                            `,`
//!
//! ___Commands___ := `{` \[ ___Command___ `,` \]* `}`
//!
//! ___Command___ := ___Name___ \[ ___Alias___ \]* \[ ___Help___ \]
//!
//! ___StructName___ /
//! ___Short___ /
//! ___Long___ /
//! ___Value___ /
//! ___Name___ /
//! ___Alias___ /
//! ___ConflictId___ := A Rust [non-keyword identifier](https://doc.rust-lang.org/reference/identifiers.html)
//!
//! ___Type___ := A Rust type that implements [`FromStr`](std::str::FromStr) + [`Debug`](std::fmt::Debug)
//!
//! ___Help___ := A Rust [string literal](https://doc.rust-lang.org/reference/tokens.html#r-lex.token.literal.str)
//!
//! ## Examples
//!
//! Options:
//!
//! ```no_rust
//! -f,                                        // Short-option
//! --foo,                                     // Long-option
//! -f --foo,                                  // Short-option + long-option
//! -f -F --foo,                               // Multiple short-options + long-option
//!
//! -f <bar> u64,                              // With u64 value named "bar"
//! --foo <bar> u64,                           // With u64 value named "bar"
//! -f --foo <bar> String,                     // With String value named "bar"
//! -f -F --foo <bar> Ipv4Addr,                // With Ipv4Addr value named "bar"
//!
//! -f                          "Help text",   // With help text
//! --foo                       "Help text",   // ...
//! -f --foo                    "Help text",   // ...
//! -f --foo <bar> u64          "Help text",   // ...
//!
//! -f                       !  "Help text",   // With default conflict-id
//! --foo                    !  "Help text",   // ...
//! -f --foo                 ?  "Help text",   // ...
//! -f --foo <bar> u64       ?  "Help text",   // ...
//! -f --foo <bar> String !A !B "Help text",   // With conflict-ids "A" and "B"
//!
//! -f...                       "Help text",   // Variadic (repeatable) option
//! --foo...                    "Help text",   // ...
//! -f --foo...                 "Help text",   // ...
//! -f --foo... <bar> u64       "Help text",   // ...
//! -f --foo... <bar> String    "Help text",   // ...
//! ```
//!
//! Non-options:
//!
//! ```no_rust
//! <foo> u64,                                 // Required argument named "foo" of type u64
//! <foo> String,                              // Required argument named "foo" of type String
//! <foo>... String,                           // Required variadic argument named "foo" of type String
//! <foo>... String "Help text",               // With help text
//! <foo>... String ! "Help text",             // With default conflict-id
//! <foo>... String !A !B "Help text",         // With conflict-ids "A" and "B"
//!
//! [<foo>] u64,                               // Optional argument name "foo" of type u64
//! [<foo>] String,                            // Optional argument name "foo" of type String
//! [<foo>...] String,                         // Optional variadic argument name "foo" of type String
//! [<foo>...] String "Help text",             // With help text
//! [<foo>...] String ! "Help text",           // With default conflict-id
//! [<foo>...] String !A !B "Help text",       // With conflict-ids "A" and "B"
//!
//! <command> [...] {                          // Required command argument
//!     add,                                   // Command "add"
//!     remove rm,                             // Command "remove" with alias "rm"
//!     list ls l,                             // Command "list" with alieses "ls" and "l"
//! }
//!
//! [<command>] [...] {                        // Optional command
//!     add,
//!     remove rm,
//!     list ls l,
//! }
//!
//! <command> [...] "Help text" {              // With help texts
//!     add         "Help text for add",
//!     remove rm   "Help text for remove",
//!     list ls l   "Help text for list",
//! }
//!
//! <command> [...] ! "Help text" {            // With default conflict-id
//!     add         "Help text for add",
//!     remove rm   "Help text for remove",
//!     list ls l   "Help text for list",
//! }
//!
//! <command> [...] !A !B "Help text" {        // With conflict-ids "A" and "B"
//!     add         "Help text for add",
//!     remove rm   "Help text for remove",
//!     list ls l   "Help text for list",
//! }
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub use error::Error;
pub use immargs_macros::immargs;
use std::collections::VecDeque;
use std::collections::vec_deque::IntoIter;
use utils::from_args;
use utils::try_from_args;

mod arg;
mod error;
mod lexer;
mod macros;
mod utils;

#[doc(hidden)]
pub mod __private {
    pub use crate::arg::Command;
    pub use crate::arg::non_option;
    pub use crate::arg::option;
    pub use crate::arg::parse;
    pub use crate::utils::bin_name;
    pub use crate::utils::from;
    pub use crate::utils::from_args;
    pub use crate::utils::from_env;
    pub use crate::utils::try_from;
    pub use crate::utils::try_from_args;
    pub use crate::utils::try_from_env;
}

/// Result returned by argument parser.
pub type Result<T> = std::result::Result<T, Error>;

/// A trait implemented by all arguments `struct`s generated by [`immargs!`].
///
/// Users of `immargs` never explicitly call methods on this trait. This trait is public
/// soley to enable [`Args::into()`] and [`Args::try_into()`] to seamelessly convert
/// (sub)command arguments into an arguments `struct`.
pub trait FromArgs: Sized {
    #[doc(hidden)]
    fn from_args(args: Args) -> Result<Self>;
}

/// Command line arguments in raw form, i.e. not yet parsed.
#[derive(Debug)]
pub struct Args(VecDeque<String>);

impl Args {
    #[inline]
    fn from_env() -> Self {
        Self(
            std::env::args_os()
                .map(|arg| match arg.into_string() {
                    Ok(arg) => arg,
                    Err(arg) => arg.to_string_lossy().into_owned(),
                })
                .collect::<VecDeque<_>>(),
        )
    }

    #[inline]
    fn from<T: IntoIterator<Item: Into<String>>>(args: T) -> Self {
        Self(
            args.into_iter()
                .map(|arg| arg.into())
                .collect::<VecDeque<_>>(),
        )
    }

    #[inline]
    fn from_vec(vec: Vec<String>) -> Self {
        Self(VecDeque::from(vec))
    }

    #[inline]
    fn set_bin_name(&mut self, bin_name: String) {
        self.0.pop_front();
        self.0.push_front(bin_name)
    }

    #[inline]
    fn peek(&mut self) -> Option<&String> {
        self.0.front()
    }

    #[inline]
    fn take(&mut self) -> String {
        self.0.pop_front().unwrap()
    }

    #[inline]
    fn pop(&mut self) -> Option<String> {
        self.0.pop_front()
    }

    /// Converts (sub)command arguments into an arguments `struct`.
    ///
    /// Example:
    ///
    /// ```no_run
    /// use immargs::immargs;
    /// use std::path::PathBuf;
    ///
    /// immargs! {
    ///     MainArgs,
    ///     -h --help           "print help message",
    ///     <command> Command {
    ///         add             "add file(s)",
    ///         remove          "remove file(s)",
    ///         list            "list file(s)",
    ///     }
    /// }
    ///
    /// immargs! {
    ///     AddArgs,
    ///     -h --help           "print help message",
    ///     <file>... PathBuf   "file(s) to add",
    /// }
    ///
    /// immargs! {
    ///     RemoveArgs,
    ///     -h --help           "print help message",
    ///     <file>... PathBuf   "file(s) to remove",
    /// }
    ///
    /// immargs! {
    ///     ListArgs,
    ///     --hidden            "include hidden files",
    ///     -h --help           "print help message",
    /// }
    ///
    /// pub fn main() {
    ///     let args = MainArgs::from_env();
    ///
    ///     match args.command {
    ///         Command::Add(args) => add(args.into()),
    ///         Command::Remove(args) => remove(args.into()),
    ///         Command::List(args) => list(args.into()),
    ///     }
    /// }
    ///
    /// fn add(args: AddArgs) {
    ///     // ...
    /// }
    ///
    /// fn remove(args: RemoveArgs) {
    ///     // ...
    /// }
    ///
    /// fn list(args: ListArgs) {
    ///     // ...
    /// }
    /// ```
    #[inline]
    pub fn into<T: FromArgs>(self) -> T {
        from_args(self)
    }

    /// Converts (sub)command arguments into an arguments `struct`.
    ///
    /// Unlike [`Args::into()`], this method returns a [`Result`] to let applications
    /// implement custom error, `--help` and `--version` handling.
    #[inline]
    pub fn try_into<T: FromArgs>(self) -> Result<T> {
        try_from_args(self)
    }
}

impl IntoIterator for Args {
    type Item = String;
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
