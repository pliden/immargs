use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

/// Errors returned by argument parser.
pub enum Error {
    /// Invalid option.
    InvalidOption {
        /// The option.
        option: String,
    },
    /// Invalid argument.
    InvalidArgument {
        /// The argument.
        arg: String,
    },
    /// Invalid command.
    InvalidCommand {
        /// The argument.
        arg: String,
    },
    /// Missing argument.
    MissingArgument {
        /// The argument.
        arg: String,
    },
    /// Missing choice.
    MissingChoice {
        /// The alternatives.
        alternatives: Vec<String>,
    },
    /// Missing value for option.
    MissingValue {
        /// The option.
        option: String,
    },
    /// Unexpected value for option.
    UnexpectedValue {
        /// The option.
        option: String,
        /// The value.
        value: String,
    },
    /// Conflicting arguments.
    ConflictingArguments {
        /// First argument.
        arg0: String,
        /// Second argument.
        arg1: String,
    },
    /// Failed to parse value.
    ParsingFailed {
        /// The value.
        value: String,
        /// The error returned by [`str::parse()`].
        error: Box<dyn std::error::Error>,
    },
    /// Version information requested. Returned if option `--version` was used.
    Version {
        /// The automatically generated version message.
        message: String,
    },
    /// Help requested. Returned if option `-h` or `--help` was used.
    Help {
        /// The automatically generated help message.
        message: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::InvalidOption { option } => {
                write!(f, "invalid option '{option}'")
            }
            Self::InvalidArgument { arg } => {
                write!(f, "invalid argument '{arg}'")
            }
            Self::InvalidCommand { arg } => {
                write!(f, "invalid command '{arg}'")
            }
            Self::MissingArgument { arg } => {
                write!(f, "missing argument '{arg}'")
            }
            Self::MissingChoice { alternatives } => {
                let alts = alternatives
                    .iter()
                    .map(|alt| format!("'{alt}'"))
                    .collect::<Vec<_>>()
                    .join(" or ");
                write!(f, "missing argument {alts}")
            }
            Self::MissingValue { option } => {
                write!(f, "missing value for option '{option}'")
            }
            Self::UnexpectedValue { option, value } => {
                write!(f, "unexpected value for option '{option}': {value}")
            }
            Self::ConflictingArguments { arg0, arg1 } => {
                write!(f, "conflicting arguments '{arg0}' and '{arg1}'")
            }
            Self::ParsingFailed { value, error } => {
                write!(f, "cannot parse argument '{value}': {error}")
            }
            Self::Version { message } => {
                write!(f, "{message}")
            }
            Self::Help { message } => {
                write!(f, "{message}")
            }
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}
