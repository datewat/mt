use std::fs;
use std::fs::File;
use std::path;
use std::path::PathBuf;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        return panic!("Too many or too little arguments provided.\tUsage: 'mt <file/dir>'");
    }

    println!("{:?}", args);

    let p = PathBuf::from(&args[1]);
    println!("{:?}", fs::canonicalize(&p));
}