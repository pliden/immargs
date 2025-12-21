use immargs::args;

#[test]
fn option_single_short_char() {
    args! {
        -f,
    }

    let args = Args::from(["test", "-f"]);
    assert!(args.f);
}

#[test]
fn option_single_short_int() {
    args! {
        -_1,
    }

    let args = Args::from(["test", "-1"]);
    assert!(args._1);
}

#[test]
fn option_multiple_shorts() {
    args! {
        -f -_1,
    }

    let args = Args::from(["test", "-1"]);
    assert!(args.f);
}

#[test]
fn option_single_long() {
    args! {
        --flag,
    }

    let args = Args::from(["test", "--flag"]);
    assert!(args.flag);
}

#[test]
fn option_multiple_longs() {
    args! {
        --flag --alias,
    }

    let args = Args::from(["test", "--alias"]);
    assert!(args.flag);
}

#[test]
fn option_short_and_long() {
    args! {
        -f --flag,
    }

    let args = Args::from(["test", "-f"]);
    assert!(args.flag);
}

#[test]
fn option_multiple_shorts_and_long() {
    args! {
        -f -F --flag,
    }

    let args = Args::from(["test", "-F"]);
    assert!(args.flag);
}

#[test]
fn option_variadic_on_value() {
    args! {
        -f --flag...,
    }

    let args = Args::from(["test", "--flag", "-f", "--flag"]);
    assert!(args.flag == 3);
}

#[test]
fn option_variadic_value() {
    args! {
        -v --value... <value> String,
    }

    let args = Args::from(["test", "--value", "hello", "-v", "world"]);
    assert!(args.value.len() == 2);
    assert!(args.value[0] == "hello");
    assert!(args.value[1] == "world");
}
