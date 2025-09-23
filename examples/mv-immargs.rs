// Example of what argument parsing for a program like "mv" could look like,
// using immargs!
//
// Demonstrates that a variadic <src..> argument can be followed by a
// non-variadic <dest>.

use immargs::immargs;
use std::path::PathBuf;

immargs! {
    Args,
    -f --force        "do not prompt before overwriting",
    --version         "print version information",
    -h --help         "print help message",
    <src>... PathBuf  "file(s) to move or rename",
    <dest> PathBuf    "destination file or directory",
}

fn main() {
    let args = Args::from_env();

    println!("mv");
    println!("  force: {:?}", args.force);
    println!("  src: {:?}", args.src);
    println!("  dest: {:?}", args.dest);
}
