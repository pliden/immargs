/// Convenience macro for on-the-spot command line argument parsing.
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
