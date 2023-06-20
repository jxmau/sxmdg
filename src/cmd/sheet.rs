use std::{process::exit, path::PathBuf, fs::File, io::{Read, Write}};
use log::{info, error, trace};

use crate::metadata::{Metadata};

pub fn sheet(path: Option<String>) {
    println!("{path:?}");
    info!("Command chosen : sheet");
    trace!("Verifying the path provided");
    let path = match path {
        Some(s) if s.ends_with(".sql")=> s,
        Some(_) => {
            println!("Path argument is invalid.");
            exit(1);
        }
        None => {
            println!("Path argument is missing.");
            exit(1);
        },
    };

    trace!("Fetching and verifying the pwd");
    let pwd = match std::env::current_dir() {
        Ok(pwd) if pwd.ends_with("migrations") => pwd,
        Ok(_) => {
            println!("Fatal Error : You need to be in the migrations directory.");
            exit(1);
        }
        Err(e) => {
            println!("Fatal error.");
            error!("Impossible to fetch the current directory path.");
            trace!(" {e} ");
            exit(1);
        },
    };

    let filepath = pwd.join(path);
    trace!("Path of the file has been joined : {filepath:?}");

    let content = read_file(filepath.clone());
    write_file(filepath, content);

}

// Read the file and check if there's already a sheet, return File
pub fn read_file(path: PathBuf) -> String {
    let mut file = match File::open(path) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to open the file.");
            trace!(" {e} ");
            exit(1);
        },
    };
    trace!("File has been opened");

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to read the file.");
            trace!(" {e} ");
            exit(1);
        }
    }
    trace!("File has been read");

    if content.starts_with("-- Table:") {
        println!("Syntax has been found in the file that would suggest that the file has already a sheet generated.");
        exit(1);
    }

    content
}

// Write the file
fn write_file(path: PathBuf, content: String) {

    let mut file = match File::create(path) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to open the file.");
            trace!(" {e} ");
            exit(1);
        },
    };
    trace!("File has been opened in write-only mode.");


    let empty_metadata = Metadata::empty();
    let mut metadata = empty_metadata.generate_metadata_string();
    metadata.push_str(&content); 
    match file.write_all(metadata.as_bytes()) {
        Ok(_) => println!("Sheet has been added to the file."),
        Err(e) =>  {
            println!("Failed to write the file.");
            error!(" {e} ");
        }
    }

    file.sync_all().unwrap();
}
