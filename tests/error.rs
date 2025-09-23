use immargs::Error;
use immargs::immargs;

#[test]
fn error_invalid_option_short() {
    immargs! {}

    let args = ImmArgs::try_from(["test", "-i"]);
    assert!(matches!(&args, Err(Error::InvalidOption { option }) if option == "-i"));
    assert!(matches!(&args, Err(e) if e.to_string() == "invalid option '-i'"));
}

#[test]
fn error_invalid_option_long() {
    immargs! {}

    let args = ImmArgs::try_from(["test", "--invalid"]);
    assert!(matches!(&args, Err(Error::InvalidOption { option }) if option == "--invalid"));
    assert!(matches!(&args, Err(e) if e.to_string() == "invalid option '--invalid'"));
}

#[test]
fn error_invalid_argument() {
    immargs! {}

    let args = ImmArgs::try_from(["test", "invalid"]);
    assert!(matches!(&args, Err(Error::InvalidArgument { arg }) if arg == "invalid"));
    assert!(matches!(&args, Err(e) if e.to_string() == "invalid argument 'invalid'"));
}

#[test]
fn error_invalid_command() {
    immargs! {
        <command> Command {
            add,
            remove,
            list,
        }
    }

    let args = ImmArgs::try_from(["test", "invalid"]);
    assert!(matches!(&args, Err(Error::InvalidCommand { arg }) if arg == "invalid"));
    assert!(matches!(&args, Err(e) if e.to_string() == "invalid command 'invalid'"));
}

#[test]
fn error_missing_argument() {
    immargs! {
        <value> String,
    }

    let args = ImmArgs::try_from(["test"]);
    assert!(matches!(&args, Err(Error::MissingArgument { arg }) if arg == "<value>"));
    assert!(matches!(&args, Err(e) if e.to_string() == "missing argument '<value>'"));
}

#[test]
fn error_missing_value() {
    immargs! {
        -f <value> String,
    }

    let args = ImmArgs::try_from(["test", "-f"]);
    assert!(matches!(&args, Err(Error::MissingValue { option }) if option == "-f"));
    assert!(matches!(&args, Err(e) if e.to_string() == "missing value for option '-f'"));
}

#[test]
fn error_unexpected_value() {
    immargs! {
        -f,
    }

    let args = ImmArgs::try_from(["test", "-f=VALUE"]);
    assert!(
        matches!(&args, Err(Error::UnexpectedValue { option, value })
            if option == "-f" && value == "VALUE"
        )
    );
    assert!(matches!(&args, Err(e) if e.to_string() == "unexpected value for option '-f': VALUE"));
}

#[test]
fn error_conflicting_arguments0() {
    immargs! {
        --feature_a                !,
        --feature_b <mph> u16      !,
        --feature_c...             !,
        --feature_d... <mph> u16   !,
        <value> String             !,
    }

    let args = ImmArgs::try_from(["test", "--feature-a", "VALUE"]);
    assert!(
        matches!(&args, Err(Error::ConflictingArguments { arg0, arg1 })
            if arg0 == "--feature-a" && arg1 == "<value>"
        )
    );
    assert!(matches!(&args, Err(e)
        if e.to_string() == "conflicting arguments '--feature-a' and '<value>'"
    ));
}

#[test]
fn error_conflicting_arguments1() {
    immargs! {
        --feature_a                !A,
        --feature_b <mph> u16      !A !C,
        --feature_c...             !B !C,
        --feature_d... <mph> u16   !B,
        <value_a> String           !A,
        [<value_b>...] String      !B !C,
    }

    let args = ImmArgs::try_from(["test", "--feature-c", "VALUE_A", "VALUE_B"]);
    assert!(
        matches!(&args, Err(Error::ConflictingArguments { arg0, arg1 })
            if arg0 == "--feature-c" && arg1 == "<value-b>"
        )
    );
    assert!(matches!(&args, Err(e)
        if e.to_string() == "conflicting arguments '--feature-c' and '<value-b>'"
    ));
}

#[test]
fn error_parsing_failed() {
    let parse_value = "ABC";
    let parse_error = parse_value.parse::<u64>().unwrap_err().to_string();

    immargs! {
        --number <number> u64,
    }

    let args = ImmArgs::try_from(["test", "--number", parse_value]);
    assert!(matches!(&args, Err(Error::ParsingFailed { value, error })
        if value == parse_value && error.to_string() == parse_error
    ));
    assert!(matches!(&args, Err(e)
        if e.to_string() == format!("cannot parse argument '{parse_value}': {parse_error}")
    ));
}
