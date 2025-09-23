// Example of what argument parsing for a program like "ls" could look like,
// using immargs!

use immargs::immargs;
use std::path::PathBuf;

immargs! {
    Args,
    -a --all             ?  "do not ignore entries starting with .",
    -l --long               "use a long listing format",
    -i --inode              "print the index number of each file",
    -R --recursive          "list subdirectories recursively",
    --version               "print version information",
    -h --help               "print help message",
    [<file>...] PathBuf  ?  "list information about the file(s)",
}

fn main() {
    let args = Args::from_env();

    println!("ls");
    println!("  all: {:?}", args.all);
    println!("  long: {:?}", args.long);
    println!("  inode: {:?}", args.inode);
    println!("  recursive: {:?}", args.recursive);
    println!("  file: {:?}", args.file);
}
