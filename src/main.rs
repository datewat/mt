use std::fs;
use std::fs::File;
use std::path;
use std::path::PathBuf;
use std::env;
use std::path::Path;
use chrono;
use clap::Parser;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Too many or too little arguments provided.\nUsage: 'mt <file/dir>'");
    }

    let p : PathBuf = PathBuf::from(&args[1]);
    if !(Path::new(&p).exists()) {
        panic!("File '{}' does not exist", args[1]);
    }

    generate_trash_info_file(p);
}

fn generate_trash_info_file(file : PathBuf) {
    //date format: yyyy-mm-ddThh:mm:ss
    let date = chrono::offset::Local::now();

    println!("{:?}", date);
}