// Example of what argument parsing for a program like "git" could look like,
// using immargs_from_env!
//
// Demonstrates short-/long-options, (sub)commands, variadic arguments,
// conflicting arguments, etc.
//
// Unlike immargs!, immargs_from_env! returns an anonymous struct and an
// anonymous command enum, which means we can't use a match-statement on
// the command enum. Instead we convert the command enum to a &str using
// into_str().

use immargs::Args;
use immargs::immargs_from;
use immargs::immargs_from_env;
use std::path::PathBuf;

fn main() {
    let args = immargs_from_env! {
        -C --dir <path> PathBuf     "set working directory",
        --version                   "print version information",
        -h --help                   "print help message",
        <command> GitCommand        "command to run" {
            clone                   "clone repository",
            add                     "add file(s)",
            move_ mv                "move or rename file(s)",
            commit co               "commit changes",
        },
    };

    println!("git");
    println!("  dir: {:?}", args.dir);
    println!("  command: {:?}", args.command);
    println!();

    match args.command.into_str() {
        ("clone", args) => git_clone(args),
        ("add", args) => git_add(args),
        ("move", args) => git_move(args),
        ("commit", args) => git_commit(args),
        _ => unreachable!(),
    }
}

fn git_clone(args: Args) {
    let args = immargs_from! {
        args,
        --progress                  "enable progress reporting",
        -n --no_checkout            "don't create a checkout",
        -h --help                   "print help message",
        <repo> String               "repository to clone",
        [<dir>] PathBuf             "target directory",
    };

    println!("git clone");
    println!("  progress:    {:?}", args.progress);
    println!("  no-checkout: {:?}", args.no_checkout);
    println!("  repo:        {:?}", args.repo);
    println!("  dir:         {:?}", args.dir);
}

fn git_add(args: Args) {
    let args = immargs_from! {
        args,
        -A --all                 ?  "add changes from all tracked and untracked files",
        -u --update              ?  "update tracked files",
        -h --help                   "print help message",
        [<pathspec>...] PathBuf  ?  "file(s) to add/update",
    };

    println!("git add");
    println!("  all:      {:?}", args.all);
    println!("  update:   {:?}", args.update);
    println!("  pathspec: {:?}", args.pathspec);
}

fn git_move(args: Args) {
    let args = immargs_from! {
        args,
        -f --force                  "force move/rename even if target exists",
        -h --help                   "print help message",
        <source>... PathBuf         "file(s) to move",
        <destination> PathBuf       "target file name or destination directory",
    };

    println!("git move");
    println!("  force:       {:?}", args.force);
    println!("  source:      {:?}", args.source);
    println!("  destination: {:?}", args.destination);
}

fn git_commit(args: Args) {
    let args = immargs_from! {
        args,
        -a --all                 ?  "commit all changed files",
        --amend                     "amend previous commit",
        -m --message <msg> String   "commit message",
        -h --help                   "print help message",
        [<pathspec>...] PathBuf  ?  "file(s) to commit",
    };

    println!("git commit");
    println!("  all:      {:?}", args.all);
    println!("  amend:    {:?}", args.amend);
    println!("  message:  {:?}", args.message);
    println!("  pathspec: {:?}", args.pathspec);
}
