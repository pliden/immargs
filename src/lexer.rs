#![doc(hidden)]

use crate::Args;
use crate::Error;
use crate::Result;
use std::mem::replace;

enum State {
    // Next argument can be an option, an option value, or a non-option
    Any,

    // Next argument is a short-option (in the middle of "-abc")
    Short { remaining: String },

    // Next argument is an option value (in the middle of "-o=value" or "--option=value")
    Value { value: String },

    // Next argument is a non-option
    None,
}

pub struct Lexer {
    state: State,
    option: String,
    args: Args,
}

const DASHDASH: &str = "--";
const DASH: &str = "-";
const EQUALS: &str = "=";
const SHORT_LENGTH: usize = 2;

impl Lexer {
    #[inline]
    pub fn new(args: Args) -> Self {
        Self {
            state: State::Any,
            option: String::new(),
            args,
        }
    }

    #[inline]
    fn next_short(&mut self, mut short: String) -> &'_ str {
        if let Some((remaining_at, _)) = short.char_indices().nth(SHORT_LENGTH) {
            let mut remaining = short.split_off(remaining_at);
            if remaining.starts_with(EQUALS) {
                remaining.remove(0);
                self.state = State::Value { value: remaining };
            } else {
                remaining.insert_str(0, DASH);
                self.state = State::Short { remaining };
            }
        }

        self.option = short;
        &self.option
    }

    #[inline]
    fn next_long(&mut self, mut long: String) -> &'_ str {
        if let Some(equals) = long.find(EQUALS) {
            let mut value = long.split_off(equals);
            value.remove(0);
            self.state = State::Value { value };
        }

        self.option = long;
        &self.option
    }

    #[inline]
    fn next_none(&mut self) {
        self.option.clear();
        self.state = State::None;
    }

    #[inline]
    pub(crate) fn next_option(&mut self) -> Result<Option<&'_ str>> {
        match replace(&mut self.state, State::Any) {
            State::Any => {
                if let Some(arg) = self.args.peek() {
                    if let Some(arg) = arg.strip_prefix(DASHDASH) {
                        if arg.is_empty() {
                            let _ = self.args.take();
                        } else {
                            let long = self.args.take();
                            return Ok(Some(self.next_long(long)));
                        }
                    } else if let Some(arg) = arg.strip_prefix(DASH)
                        && !arg.is_empty()
                    {
                        let short = self.args.take();
                        return Ok(Some(self.next_short(short)));
                    }
                }

                self.next_none();
                Ok(None)
            }
            State::Short { remaining } => Ok(Some(self.next_short(remaining))),
            State::Value { value } => Err(Error::UnexpectedValue {
                option: self.option.clone(),
                value,
            }),
            State::None => panic!(),
        }
    }

    #[inline]
    pub(crate) fn next_value(&mut self) -> Result<String> {
        match replace(&mut self.state, State::Any) {
            State::Any => match self.args.pop() {
                Some(value) => {
                    assert!(!self.option.is_empty());
                    self.option.clear();
                    Ok(value)
                }
                _ => Err(Error::MissingValue {
                    option: self.option.clone(),
                }),
            },
            State::Short { mut remaining } => {
                remaining.remove(0);
                Ok(remaining)
            }
            State::Value { value } => Ok(value),
            State::None => panic!(),
        }
    }

    #[inline]
    pub(crate) fn non_options(&mut self) -> Result<&mut Args> {
        if let State::Any = self.state
            && let Some(_) = self.next_option()?
        {
            panic!();
        };

        match self.state {
            State::None => Ok(&mut self.args),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Args;
    use super::Error;
    use super::Lexer;

    macro_rules! assert_none {
        ($lexer:ident) => {
            assert!(matches!($lexer.next_option(), Ok(None)));
        };
    }

    macro_rules! assert_option {
        ($lexer:ident, $option:literal) => {
            assert!(matches!($lexer.next_option(), Ok(Some($option))));
        };
    }

    macro_rules! assert_value {
        ($lexer:ident, $value:literal) => {
            assert!(matches!($lexer.next_value(), Ok(value) if value == $value));
        };
    }

    macro_rules! assert_non_options {
        ($lexer:ident, $non_options:expr) => {
            assert!($lexer.non_options().unwrap().0 == Args::from($non_options).0);
        };
    }

    #[test]
    fn short() {
        let mut l = Lexer::new(Args::from(["-s"]));
        assert_option!(l, "-s");
        assert_none!(l);
    }

    #[test]
    fn short_space_value() {
        let mut l = Lexer::new(Args::from(["-s", "VALUE"]));
        assert_option!(l, "-s");
        assert_value!(l, "VALUE");
        assert_none!(l);
    }

    #[test]
    fn short_attached_value() {
        let mut l = Lexer::new(Args::from(["-sVALUE"]));
        assert_option!(l, "-s");
        assert_value!(l, "VALUE");
        assert_none!(l);
    }

    #[test]
    fn short_equals_value() {
        let mut l = Lexer::new(Args::from(["-s=VALUE"]));
        assert_option!(l, "-s");
        assert_value!(l, "VALUE");
        assert_none!(l);
    }

    #[test]
    fn short_space_empty_value() {
        let mut l = Lexer::new(Args::from(["-s", ""]));
        assert_option!(l, "-s");
        assert_value!(l, "");
        assert_none!(l);
    }

    #[test]
    fn short_equals_empty_value() {
        let mut l = Lexer::new(Args::from(["-s="]));
        assert_option!(l, "-s");
        assert_value!(l, "");
        assert_none!(l);
    }

    #[test]
    fn short_combined() {
        let mut l = Lexer::new(Args::from(["-abc"]));
        assert_option!(l, "-a");
        assert_option!(l, "-b");
        assert_option!(l, "-c");
        assert_none!(l);
    }

    #[test]
    fn short_combined_space_value() {
        let mut l = Lexer::new(Args::from(["-abc", "VALUE"]));
        assert_option!(l, "-a");
        assert_option!(l, "-b");
        assert_option!(l, "-c");
        assert_value!(l, "VALUE");
        assert_none!(l);
    }

    #[test]
    fn short_combined_attached_value() {
        let mut l = Lexer::new(Args::from(["-abcVALUE"]));
        assert_option!(l, "-a");
        assert_option!(l, "-b");
        assert_option!(l, "-c");
        assert_value!(l, "VALUE");
        assert_none!(l);
    }

    #[test]
    fn short_combined_equals_value() {
        let mut l = Lexer::new(Args::from(["-abc=VALUE"]));
        assert_option!(l, "-a");
        assert_option!(l, "-b");
        assert_option!(l, "-c");
        assert_value!(l, "VALUE");
        assert_none!(l);
    }

    #[test]
    fn short_combined_equals_empty_value() {
        let mut l = Lexer::new(Args::from(["-abc="]));
        assert_option!(l, "-a");
        assert_option!(l, "-b");
        assert_option!(l, "-c");
        assert_value!(l, "");
        assert_none!(l);
    }

    #[test]
    fn long() {
        let mut l = Lexer::new(Args::from(["--long"]));
        assert_option!(l, "--long");
        assert_none!(l);
    }

    #[test]
    fn long_space_value() {
        let mut l = Lexer::new(Args::from(["--long", "VALUE"]));
        assert_option!(l, "--long");
        assert_value!(l, "VALUE");
        assert_none!(l);
    }

    #[test]
    fn long_equals_value() {
        let mut l = Lexer::new(Args::from(["--long=VALUE"]));
        assert_option!(l, "--long");
        assert_value!(l, "VALUE");
        assert_none!(l);
    }

    #[test]
    fn long_equals_empty_value() {
        let mut l = Lexer::new(Args::from(["--long="]));
        assert_option!(l, "--long");
        assert_value!(l, "");
        assert_none!(l);
    }

    #[test]
    fn non_options() {
        let mut l = Lexer::new(Args::from(["abc", "-abc", "--abc"]));
        assert_none!(l);
        assert_non_options!(l, ["abc", "-abc", "--abc"]);
    }

    #[test]
    fn non_options_dash() {
        let mut l = Lexer::new(Args::from(["-", "abc", "-abc", "--abc"]));
        assert_none!(l);
        assert_non_options!(l, ["-", "abc", "-abc", "--abc"]);
    }

    #[test]
    fn non_options_after_dashdash() {
        let mut l = Lexer::new(Args::from(["--", "abc", "-abc", "--abc"]));
        assert_none!(l);
        assert_non_options!(l, ["abc", "-abc", "--abc"]);
    }

    #[test]
    fn non_options_includes_dashdash() {
        let mut l = Lexer::new(Args::from(["abc", "--", "-abc", "--abc"]));
        assert_none!(l);
        assert_non_options!(l, ["abc", "--", "-abc", "--abc"]);
    }

    #[test]
    fn mixed() {
        let mut l = Lexer::new(Args::from([
            "-f",
            "-xyz",
            "-g=5",
            "-h=",
            "-i32",
            "-j",
            "",
            "-=",
            "-=X",
            "--color",
            "red",
            "--title=",
            "--age=47",
            "-",
            "start",
            "file.txt",
            "-s",
            "-s=VALUE",
            "--long",
            "--long=VALUE",
        ]));

        assert_option!(l, "-f");
        assert_option!(l, "-x");
        assert_option!(l, "-y");
        assert_option!(l, "-z");
        assert_option!(l, "-g");
        assert_value!(l, "5");
        assert_option!(l, "-h");
        assert_value!(l, "");
        assert_option!(l, "-i");
        assert_value!(l, "32");
        assert_option!(l, "-j");
        assert_value!(l, "");
        assert_option!(l, "-=");
        assert_option!(l, "-=");
        assert_option!(l, "-X");
        assert_option!(l, "--color");
        assert_value!(l, "red");
        assert_option!(l, "--title");
        assert_value!(l, "");
        assert_option!(l, "--age");
        assert_value!(l, "47");
        assert_none!(l);
        assert_non_options!(
            l,
            [
                "-",
                "start",
                "file.txt",
                "-s",
                "-s=VALUE",
                "--long",
                "--long=VALUE"
            ]
        );
    }

    #[test]
    fn error_missing_value() {
        let mut l = Lexer::new(Args::from(["-s"]));
        assert_option!(l, "-s");
        assert!(matches!(
            l.next_value(),
            Err(Error::MissingValue { option }) if option == "-s"
        ));
    }

    #[test]
    fn error_unexpected_value() {
        let mut l = Lexer::new(Args::from(["-s=VALUE"]));
        assert_option!(l, "-s");
        assert!(matches!(
            l.next_option(),
            Err(Error::UnexpectedValue { option, value }) if option == "-s" && value == "VALUE"
        ));
    }
}
