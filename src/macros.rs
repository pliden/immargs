/// Convenience macro for on-the-spot command line argument parsing.
///
/// Parses arguments provided by caller, returns a [`Result`]`<T>`,
/// where `T` is an anonymous `struct`.
///
/// Example:
///
/// ```
/// use immargs::immargs_try_from;
///
/// let cmd_line_args = ["test", "-l8", "src", "dest"];
///
/// let args = immargs_try_from! {
///     cmd_line_args,
///     --force               "overwrite destination",
///     -l --log <level> u8   "set log level",
///     -h --help             "print help message",
///     <src>... String       "source(s)",
///     <dest> String         "destination",
/// };
///```
#[macro_export]
macro_rules! immargs_try_from {
    ($args:expr, $($spec:tt)*) => {{
        ::immargs::immargs! {
            ImmArgs,
            $($spec)*
        }
        ImmArgs::try_from($args)
    }};
}

/// Convenience macro for on-the-spot command line argument parsing.
///
/// Parses arguments provided by [`std::env::args_os()`], returns a [`Result`]`<T>`,
/// where `T` is an anonymous `struct`.
///
/// Example:
///
/// ```no_run
/// use immargs::immargs_try_from_env;
///
/// let args = immargs_try_from_env! {
///     --force               "overwrite destination",
///     -l --log <level> u8   "set log level",
///     -h --help             "print help message",
///     <src>... String       "source(s)",
///     <dest> String         "destination",
/// };
/// ```
#[macro_export]
macro_rules! immargs_try_from_env {
    ($($spec:tt)*) => {{
        ::immargs::immargs! {
            ImmArgs,
            $($spec)*
        }
        ImmArgs::try_from_env()
    }};
}

/// Convenience macro for on-the-spot command line argument parsing.
///
/// Parses arguments provided by the caller, returns an anonymous `struct`.
///
/// Example:
///
/// ```
/// use immargs::immargs_from;
///
/// let cmd_line_args = ["test", "-l8", "src", "dest"];
///
/// let args = immargs_from! {
///     cmd_line_args,
///     --force               "overwrite destination",
///     -l --log <level> u8   "set log level",
///     -h --help             "print help message",
///     <src>... String       "source(s)",
///     <dest> String         "destination",
/// };
/// ```
#[macro_export]
macro_rules! immargs_from {
    ($args:expr, $($spec:tt)*) => {{
        ::immargs::immargs! {
            ImmArgs,
            $($spec)*
        }
        ImmArgs::from($args)
    }};
}

/// Convenience macro for on-the-spot command line argument parsing.
///
/// Parses arguments provided by [`std::env::args_os()`], returns an
/// anonymous `struct`.
///
/// Example:
///
/// ```no_run
/// use immargs::immargs_from_env;
///
/// let args = immargs_from_env! {
///     --force               "overwrite destination",
///     -l --log <level> u8   "set log level",
///     -h --help             "print help message",
///     <src>... String       "source(s)",
///     <dest> String         "destination",
/// };
/// ```
#[macro_export]
macro_rules! immargs_from_env {
    ($($spec:tt)*) => {{
        ::immargs::immargs! {
            ImmArgs,
            $($spec)*
        }
        ImmArgs::from_env()
    }};
}
