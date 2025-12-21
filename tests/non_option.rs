use immargs::args;

#[test]
fn non_option_required() {
    args! {
        <hello> String,
        <world> String,
    }

    let args = ImmArgs::from(["test", "hello", "world"]);
    assert!(args.hello == "hello");
    assert!(args.world == "world");
}

#[test]
fn non_option_optional() {
    args! {
        [<hello>] String,
        [<world>] String,
    }

    let args = ImmArgs::from(["test", "hello"]);
    assert!(matches!(args.hello, Some(s) if s == "hello"));
    assert!(args.world.is_none());
}

#[test]
fn non_option_mixed() {
    args! {
        <value0> String,
        <value1> String,
        [<value2>] String,
        [<value3>] String,
    }

    let args = ImmArgs::from(["test", "required0", "required1", "optional0"]);
    assert!(args.value0 == "required0");
    assert!(args.value1 == "required1");
    assert!(matches!(args.value2, Some(s) if s == "optional0"));
    assert!(args.value3.is_none());
}

#[test]
fn non_option_required_variadic() {
    args! {
        <value>... String,
    }

    let args = ImmArgs::from(["test", "hello", "world"]);
    assert!(args.value.len() == 2);
    assert!(args.value[0] == "hello");
    assert!(args.value[1] == "world");
}

#[test]
fn non_option_optional_variadic() {
    args! {
        [<value>...] String,
    }

    let args = ImmArgs::from(["test", "hello", "world"]);
    assert!(args.value.len() == 2);
    assert!(args.value[0] == "hello");
    assert!(args.value[1] == "world");
}

#[test]
fn non_option_required_variadic_redistribute0() {
    args! {
        <value0> u64,
        <value1>... String,
        <value2> u64,
    }

    let args = ImmArgs::from(["test", "0", "1", "2"]);
    assert!(args.value0 == 0);
    assert!(args.value1.len() == 1);
    assert!(args.value1[0] == "1");
    assert!(args.value2 == 2);
}

#[test]
fn non_option_required_variadic_redistribute1() {
    args! {
        <value0>... u64,
        <value1> u64,
        <value2> u64,
    }

    let args = ImmArgs::from(["test", "0", "1", "2", "3", "4"]);
    assert!(args.value0.len() == 3);
    assert!(args.value0[0] == 0);
    assert!(args.value0[1] == 1);
    assert!(args.value0[2] == 2);
    assert!(args.value1 == 3);
    assert!(args.value2 == 4);
}

#[test]
fn non_option_optional_variadic_redistribute0() {
    args! {
        [<value0>...] u64,
        [<value1>] u64,
        [<value2>] u64,
    }

    let args = ImmArgs::from(["test"]);
    assert!(args.value0.is_empty());
    assert!(args.value1.is_none());
    assert!(args.value2.is_none());
}

#[test]
fn non_option_optional_variadic_redistribute1() {
    args! {
        [<value0>...] u64,
        [<value1>] u64,
        [<value2>] u64,
    }

    let args = ImmArgs::from(["test", "0"]);
    assert!(args.value0.len() == 1);
    assert!(args.value0[0] == 0);
    assert!(args.value1.is_none());
    assert!(args.value2.is_none());
}

#[test]
fn non_option_optional_variadic_redistribute2() {
    args! {
        [<value0>...] u64,
        [<value1>] u64,
        [<value2>] u64,
    }

    let args = ImmArgs::from(["test", "0", "1"]);
    assert!(args.value0.len() == 1);
    assert!(args.value0[0] == 0);
    assert!(args.value1 == Some(1));
    assert!(args.value2.is_none());
}

#[test]
fn non_option_mixed_variadic_redistribute() {
    args! {
        <value0>... u64,
        <value1> u64,
        [<value2>] u64,
        [<value3>] u64,
    }

    let args = ImmArgs::from(["test", "0", "1", "2"]);
    assert!(args.value0.len() == 1);
    assert!(args.value0[0] == 0);
    assert!(args.value1 == 1);
    assert!(args.value2 == Some(2));
    assert!(args.value3.is_none());
}

#[test]
fn non_option_command_required() {
    args! {
        <command> Command {
            add,
            remove,
            list,
        },
    }

    let args = ImmArgs::from(["test", "list", "arg0", "arg1"]);
    let Command::List(args) = args.command else {
        panic!();
    };
    let mut args = args.into_iter();
    assert!(args.next().unwrap() == "test list");
    assert!(args.next().unwrap() == "arg0");
    assert!(args.next().unwrap() == "arg1");
    assert!(args.next().is_none());
}

#[test]
fn non_option_command_optional() {
    args! {
        [<command>] Command {
            add,
            remove,
            list,
        },
    }

    let args = ImmArgs::from(["test", "list", "arg0", "arg1"]);
    let Some(Command::List(args)) = args.command else {
        panic!();
    };
    let mut args = args.into_iter();
    assert!(args.next().unwrap() == "test list");
    assert!(args.next().unwrap() == "arg0");
    assert!(args.next().unwrap() == "arg1");
    assert!(args.next().is_none());
}

#[test]
fn non_option_command_alias() {
    args! {
        <command> Command {
            add,
            remove,
            list ls l,
        },
    }

    let args = ImmArgs::from(["test", "ls", "arg0", "arg1"]);
    let Command::List(args) = args.command else {
        panic!();
    };
    let mut args = args.into_iter();
    assert!(args.next().unwrap() == "test list");
    assert!(args.next().unwrap() == "arg0");
    assert!(args.next().unwrap() == "arg1");
    assert!(args.next().is_none());
}
