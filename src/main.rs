use std::{fs, fs::File, path::{Path, PathBuf}, env, io::Write};
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

    fill_trash_info(trash_info.unwrap(), file);
}

fn fill_trash_info(mut file: File, path: PathBuf) {
    //date format: yyyy-mm-ddThh:mm:ss
    let date = Local::now().format("%Y-%m-%dT%H:%M:%S");

    let _homedir = env::var_os("HOME").unwrap();
    let homedir = _homedir.to_str().unwrap();

    let _pwd = env::var_os("PWD").unwrap();
    let pwd = _pwd.to_str().unwrap();

    let first_line= "[Trash Info]\n";
    let second_line = format!("Path={}\n", format!("{pwd}/{}", path.display()));
    let third_line = format!("DeletionDate={}", date);

    file.write_all(first_line.as_bytes());
    file.write_all(second_line.as_bytes());
    file.write_all(third_line.as_bytes());
}