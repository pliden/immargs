use immargs::Args;
use immargs::immargs_from;
use immargs::immargs_from_env;
use immargs::immargs_try_from;
use immargs::immargs_try_from_env;

#[test]
fn macro_immargs_from() {
    let args = immargs_from! {
        ["test", "-f", "list", "-v", "47"],
        -f --flag,
        <command> Command {
            list ls,
            add,
            remove rm,
        }
    };

    assert!(args.flag);

    match args.command.into_str() {
        ("list", args) => list(args),
        _ => panic!(),
    }

    fn list(args: Args) {
        let args = immargs_from! {
            args,
            -v --value <num> u64,
        };

        assert!(args.value == Some(47));
    }
}

#[test]
fn macro_immargs_try_from() {
    let args = immargs_try_from! {
        ["test", "-f", "list"],
        -f --flag,
        <value> String,
    };

    let args = args.unwrap();

    assert!(args.flag);
    assert!(args.value == "list");
}

#[test]
fn macro_immargs_from_env() {
    let args = immargs_from_env! {
        -f --flag,
    };

    assert!(!args.flag);
}

#[test]
fn macro_immargs_try_from_env() {
    let args = immargs_try_from_env! {
        -f --flag,
    };

    let args = args.unwrap();

    assert!(!args.flag);
}
