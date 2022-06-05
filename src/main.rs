use std::{fs, fs::File, path::{Path, PathBuf}, env};
use chrono::Local;

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

fn generate_trash_info_file(file: PathBuf) {
    let trashinfo_file_name: &String= &format!("{}.trashinfo", file.display());

    if fs::metadata(trashinfo_file_name).is_ok() {
        println!("{} file exists", trashinfo_file_name);
        return;
    }

    let trash_info = File::create(trashinfo_file_name);
    println!("{} file created", trashinfo_file_name);

    fill_trash_info(trash_info);
}

fn fill_trash_info(file: File) {
    //date format: yyyy-mm-ddThh:mm:ss
    let date = Local::now().format("%Y-%m-%dT%H:%M:%S");

    let first_line = "[Trash Info]";
    let second_line = format!("Path=");
    let third_line = format!("DeletionDate={}", date);
}