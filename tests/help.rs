use immargs::Error;
use immargs::immargs;
use indoc::indoc;

macro_rules! assert_help {
    ($result:expr, $help:expr) => {{
        let Err(Error::Help { message }) = $result else {
            panic!("should be a help error");
        };
        println!("[\n{message}]");
        assert!(message == $help);
    }};
}

#[test]
fn help_empty() {
    immargs! {
        -h --help   "Print help message",
    }

    let help = indoc! {"
        usage: test [options]

        options:
           -h, --help     Print help message

    "};

    let args = ImmArgs::try_from(["test", "-h"]);
    assert_help!(&args, help);
}

#[test]
fn help_option() {
    immargs! {
        --aaa                   "Help aaa",
        -b --bbb                "Help bbb",
        -x -y -z --ccc          "Help ccc",
        --ddd <value> String    "Help ddd",
        -e --eee <value> String "Help eee",
        -h --help               "Print help message",
    }

    let help = indoc! {"
        usage: test [options]

        options:
           --aaa                 Help aaa
           -b, --bbb             Help bbb
           -x, -y, -z, --ccc     Help ccc
           --ddd <value>         Help ddd
           -e, --eee <value>     Help eee
           -h, --help            Print help message

    "};

    let args = ImmArgs::try_from(["test", "-h"]);
    assert_help!(&args, help);
}

#[test]
fn help_non_option_required() {
    immargs! {
        -h --help        "Print help message",
        <aaa> String     "Help aaa",
        <bbb>... String  "Help bbb",
    }

    let help = indoc! {"
        usage: test [options] <aaa> <bbb>...

        options:
           -h, --help     Print help message

        arguments:
           <aaa>          Help aaa
           <bbb>...       Help bbb

    "};

    let args = ImmArgs::try_from(["test", "-h"]);
    assert_help!(&args, help);
}

#[test]
fn help_non_option_optional() {
    immargs! {
        -h --help          "Print help message",
        [<aaa>] String     "Help aaa",
        [<bbb>...] String  "Help bbb",
    }

    let help = indoc! {"
        usage: test [options] [<aaa>] [<bbb>...]

        options:
           -h, --help     Print help message

        arguments:
           [<aaa>]        Help aaa
           [<bbb>...]     Help bbb

    "};

    let args = ImmArgs::try_from(["test", "-h"]);
    assert_help!(&args, help);
}

#[test]
fn help_non_option_no_help() {
    immargs! {
        -h --help        "Print help message",
        <aaa> String,
        [<bbb>...] String,
    }

    let help = indoc! {"
        usage: test [options] <aaa> [<bbb>...]

        options:
           -h, --help     Print help message

    "};

    let args = ImmArgs::try_from(["test", "-h"]);
    assert_help!(&args, help);
}

#[test]
fn help_non_option_command() {
    immargs! {
        -h --help           "Print help message",
        <command> Command   "This is a command" {
            add             "Add file(s)",
            remove rm       "Remove file(s)",
            list ls l       "List file(s)",
        },
    }

    let help = indoc! {"
        usage: test [options] <command> [...]

        options:
           -h, --help      Print help message

        arguments:
           <command>       This is a command

        commands:
           add             Add file(s)
           remove, rm      Remove file(s)
           list, ls, l     List file(s)

    "};

    let args = ImmArgs::try_from(["test", "-h"]);
    assert_help!(&args, help);
}
