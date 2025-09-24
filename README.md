# immargs - Immediate Arguments

[![Crates.io](https://img.shields.io/crates/v/immargs?logo=rust&label=Crates.io&labelColor=black)](https://crates.io/crates/immargs)
[![Docs.rs](https://img.shields.io/docsrs/immargs?logo=rust&label=Docs.rs&labelColor=black)](https://docs.rs/immargs/)
[![Built & Test](https://img.shields.io/github/actions/workflow/status/pliden/immargs/build-test.yaml?logo=github&label=Build%20%26%20Test&labelColor=black)](https://github.com/pliden/immargs/actions/workflows/build-test.yaml)

__No-hassle, on-the-spot, command line argument parsing for Rust__

Highlights:

* Straightforward declaration of arguments with proc-macro.
* Supports [POSIX](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap12.html) /
[GNU](https://sourceware.org/glibc/manual/latest/html_node/Argument-Syntax.html)
argument syntax conventions.
* Supports arguments of any type that implements `FromStr` + `Debug`.
* Supports (sub)commands, with aliases.
* Supports declaration of conflicting arguments.
* Supports automatic `--version` and `--help` handling, with possibility to opt-out.
* Tested on Linux, macOS and Windows.
* No run-time dependencies.

[Full documentation](https://docs.rs/immargs/)

## Basic Example

Using `immargs_from_env!` for on-the-spot declaration and parsing of command line
arguments. Returns an anonymous `struct` with fields corresponding to the declared
arguments.

```rust
use immargs::immargs_from_env;

let args = immargs_from_env! {
    --force               "overwrite destination",
    -l --log <level> u8   "set log level",
    -h --help             "print help message",
    <src>... String       "source(s)",
    <dest> String         "destination",
};

// Assuming this program was executed with "myprog -l 3 Src0 Src1 Dest"
assert!(!args.force);
assert!(args.log == Some(3));
assert!(args.src.len() == 2);
assert!(args.src[0] == "Src0");
assert!(args.src[1] == "Src1");
assert!(args.dest == "Dest");
```

## Advanced Example

Using `immargs!` for declaring command line arguments.

```rust
use immargs::immargs;

immargs! {
    MainArgs,
    -v --verbose           "enable verbose logging",
    --version              "print version",
    -h --help              "print help message",
    <command> Command      "the command to run" {     // Command enum will be named "Command"
        add                "add file(s)",
        remove rm          "remove file(s)",
        commit co c        "commit changes",
    }
}

immargs! {
    AddArgs,
    -a --all            ?  "add all files",           // Conflicts with [<file>...]
    --force                "overwrite destination",
    -h --help              "print help message",
    [<file>...] String  ?  "file(s) to add",          // Conflicts with -a, --all
}

immargs! {
    RemoveArgs,
    -r --recursive         "recursively remove files",
    -h --help              "print help message",
    <file>... String       "file(s) to remove",       // "..." means it's a variadic argument
}

immargs!(
    CommitArgs,
    -a --amend             "amend latest commit",
    -h --help              "print help message",
    [<message>] String     "commit message",          // "[ ]" means it's an optional argument
);

fn main() {
    let main_args = MainArgs::from_env();
    let verbose = main_args.verbose;

    match main_args.command {
        Command::Add(args) => add(verbose, args.into()),
        Command::Remove(args) => remove(verbose, args.into()),
        Command::Commit(args) => commit(verbose, args.into()),
    }
}

fn add(verbose: bool, args: AddArgs) {
    // ...
}

fn remove(verbose: bool, args: RemoveArgs) {
    // ...
}

fn commit(verbose: bool, args: CommitArgs) {
    // ...
}
```

## Command Line Argument Syntax

The following
[POSIX](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap12.html) /
[GNU](https://sourceware.org/glibc/manual/latest/html_node/Argument-Syntax.html)
argument syntax conventions are supported:
* Short option, `-` followed by a single character, e.g. `-f`.
* Long option, `--` followed by two or more characters, e.g. `--foo`.
* Short/Long option with separate value, e.g. `-f 100` or `--foo 100`.
* Short/Long option with attached value delimited by `=`, e.g. `-f=100` or `--foo=100`.
* Short option with attached value without delimiter, e.g. `-f100`.
* Combined short options, e.g. `-abc` is equivalent to `-a -b -c`.
* Short/Long options may appear in any order, but must come before any non-option arguments.
* Short/Long options may appear multiple times, the last appearance takes precedence unless
  it's a _variadic_ (repeatable) option, where the number of times the option appears has
  meaning, e.g. `-vvv` where each `-v` increases the verbosity level.
* The order of non-option arguments carries meaning.
* A standalone `-` argument is treated as a non-option argument.
* A standalone `--` argument marks the end of options. Any following arguments are treated
  as non-option arguments.

## License

Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT License](LICENSE-MIT)

at your option.

## Contribution

Any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
