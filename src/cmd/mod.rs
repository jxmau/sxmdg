use std::{path::{PathBuf}};
use log::{error, trace};
use std::process::exit;

pub mod generate;
pub mod sheet;


// Get all sql files from dir
pub fn get_sqlx_files_from_dir(path: &PathBuf) -> Vec<(String, PathBuf)> {
trace!("Dir to read : {path:?} ");
let files = match path.read_dir() {
    Ok(s) => s,
    Err(e) => {
        println!("Fatal error");
        error!("Failed to read the entries of the directory given.");
        trace!(" {e} ");
        exit(1);
    }
};

let mut list_sent: Vec<(String, PathBuf)> = Vec::new();
for file in files {
    if let Ok(valid_file) = file {
        if let Ok(valid_filename) = valid_file.file_name().into_string() {
            if valid_filename.ends_with(".sql") {
                list_sent.push((valid_filename, valid_file.path()));
            }
        }
    } 
};

list_sent
}