// Example of what argument parsing using immargs could look like for a program
// like "mv". Demonstrates that a variadic <src..> argument can be followed by a
// non-variadic <dest>.

use immargs::immargs_from_env;
use std::path::PathBuf;

fn main() {
    let args = immargs_from_env! {
        -f --force        "do not prompt before overwriting",
        --version         "print version information",
        -h --help         "print help message",
        <src>... PathBuf  "file(s) to move or rename",
        <dest> PathBuf    "destination file or directory",
    };

    println!("mv");
    println!("  force: {:?}", args.force);
    println!("  src: {:?}", args.src);
    println!("  dest: {:?}", args.dest);
}
