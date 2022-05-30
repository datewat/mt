use std::fs;
use std::fs::File;
use std::path;
use std::path::PathBuf;
use std::env;
use std::path::Path;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Too many or too little arguments provided.\nUsage: 'mt <file/dir>'");
    }

    println!("{:?}", args);

    let p = PathBuf::from(&args[1]);
    if !(Path::new(&p).exists()) {
        panic!("File '{}' does not exist", args[1]);
    }
}