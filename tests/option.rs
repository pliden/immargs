use immargs::immargs;

#[test]
fn option_single_short_char() {
    immargs! {
        -f,
    }

    let args = ImmArgs::from(["test", "-f"]);
    assert!(args.f);
}

#[test]
fn option_single_short_int() {
    immargs! {
        -_1,
    }

    let args = ImmArgs::from(["test", "-1"]);
    assert!(args._1);
}

#[test]
fn option_multiple_shorts() {
    immargs! {
        -f -_1,
    }

    let args = ImmArgs::from(["test", "-1"]);
    assert!(args.f);
}

#[test]
fn option_single_long() {
    immargs! {
        --flag,
    }

    let args = ImmArgs::from(["test", "--flag"]);
    assert!(args.flag);
}

#[test]
fn option_multiple_longs() {
    immargs! {
        --flag --alias,
    }

    let args = ImmArgs::from(["test", "--alias"]);
    assert!(args.flag);
}

#[test]
fn option_short_and_long() {
    immargs! {
        -f --flag,
    }

    let args = ImmArgs::from(["test", "-f"]);
    assert!(args.flag);
}

#[test]
fn option_multiple_shorts_and_long() {
    immargs! {
        -f -F --flag,
    }

    let args = ImmArgs::from(["test", "-F"]);
    assert!(args.flag);
}

#[test]
fn option_variadic_on_value() {
    immargs! {
        -f --flag...,
    }

    let args = ImmArgs::from(["test", "--flag", "-f", "--flag"]);
    assert!(args.flag == 3);
}

#[test]
fn option_variadic_value() {
    immargs! {
        -v --value... <value> String,
    }

    let args = ImmArgs::from(["test", "--value", "hello", "-v", "world"]);
    assert!(args.value.len() == 2);
    assert!(args.value[0] == "hello");
    assert!(args.value[1] == "world");
}
