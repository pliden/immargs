// Example of what argument parsing for a program like "ls" could look like,
// using args_from_env!

use immargs::args_from_env;
use std::path::PathBuf;

fn main() {
    let args = args_from_env! {
        -a --all             ?  "do not ignore entries starting with .",
        -l --long               "use a long listing format",
        -i --inode              "print the index number of each file",
        -R --recursive          "list subdirectories recursively",
        --version               "print version information",
        -h --help               "print help message",
        [<file>...] PathBuf  ?  "list information about the file(s)",
    };

    println!("ls");
    println!("  all: {:?}", args.all);
    println!("  long: {:?}", args.long);
    println!("  inode: {:?}", args.inode);
    println!("  recursive: {:?}", args.recursive);
    println!("  file: {:?}", args.file);
}
