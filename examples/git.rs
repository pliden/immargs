// Example of what argument parsing using immargs could look like for a program
// like "git". Demonstrates short-/long-options, (sub)commands, variadic arguments,
// conflicting arguments, etc.

use immargs::immargs;
use std::path::PathBuf;

immargs! {
    MainArgs,
    -C --dir <path> PathBuf     "set working directory",
    --version                   "print version information",
    -h --help                   "print help message",
    <command> GitCommand        "command to run" {
        clone                   "clone repository",
        add                     "add file(s)",
        move_ mv                "move or rename file(s)",
        commit co               "commit changes",
    },
}

immargs! {
    CloneArgs,
    --progress                  "enable progress reporting",
    -n --no_checkout            "don't create a checkout",
    -h --help                   "print help message",
    <repo> String               "repository to clone",
    [<dir>] PathBuf             "target directory",
}

immargs! {
    AddArgs,
    -A --all                 ?  "add changes from all tracked and untracked files",
    -u --update              ?  "update tracked files",
    -h --help                   "print help message",
    [<pathspec>...] PathBuf  ?  "file(s) to add/update",
}

immargs! {
    MoveArgs,
    -f --force                  "force move/rename even if target exists",
    -h --help                   "print help message",
    <source>... PathBuf         "file(s) to move",
    <destination> PathBuf       "target file name or destination directory",
}

immargs! {
    CommitArgs,
    -a --all                 ?  "commit all changed files",
    --amend                     "amend previous commit",
    -m --message <msg> String   "commit message",
    -h --help                   "print help message",
    [<pathspec>...] PathBuf  ?  "file(s) to commit",
}

fn main() {
    let args = MainArgs::from_env();

    println!("git");
    println!("  dir: {:?}", args.dir);
    println!("  command: {:?}", args.command);
    println!();

    match args.command {
        GitCommand::Clone(args) => git_clone(args.into()),
        GitCommand::Add(args) => git_add(args.into()),
        GitCommand::Move(args) => git_move(args.into()),
        GitCommand::Commit(args) => git_commit(args.into()),
    }
}

fn git_clone(args: CloneArgs) {
    println!("git clone");
    println!("  progress:    {:?}", args.progress);
    println!("  no-checkout: {:?}", args.no_checkout);
    println!("  repo:        {:?}", args.repo);
    println!("  dir:         {:?}", args.dir);
}

fn git_add(args: AddArgs) {
    println!("git add");
    println!("  all:      {:?}", args.all);
    println!("  update:   {:?}", args.update);
    println!("  pathspec: {:?}", args.pathspec);
}

fn git_move(args: MoveArgs) {
    println!("git move");
    println!("  force:       {:?}", args.force);
    println!("  source:      {:?}", args.source);
    println!("  destination: {:?}", args.destination);
}

fn git_commit(args: CommitArgs) {
    println!("git commit");
    println!("  all:      {:?}", args.all);
    println!("  amend:    {:?}", args.amend);
    println!("  message:  {:?}", args.message);
    println!("  pathspec: {:?}", args.pathspec);
}
