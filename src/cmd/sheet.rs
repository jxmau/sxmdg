use std::{process::exit, path::PathBuf, fs::{File}, io::{Read, Write}};
use log::{info, error, trace};

use crate::{metadata::{Metadata}, cmd::get_sqlx_files_from_dir};

pub fn sheet(path: Option<String>) {
    
    info!("Command chosen : sheet");
    
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


    trace!("Verifying the path provided");
    let mut files : Vec<(String, PathBuf)> = Vec::new();
    match path {
        Some(s) if s.ends_with(".sql")=> {
            info!("One file to add sheet to.");
            let mut path = PathBuf::new();
            path.set_file_name(pwd.join(s.clone()));
            files.push((s, path))
        },
        Some(s) if s.eq("--all") | s.eq("-a") => { 
            let mut files_read = get_sqlx_files_from_dir(&pwd);
            files.append(&mut files_read );
        }
        Some(_) => {
            println!("Path argument is invalid.");
            exit(1);
        }
        None => {
            println!("Path argument is missing.");
            exit(1);
        },
    };


    for file in files {
        let (file_name, file_path) = file;
        trace!("Path of the file : {file_path:?}");
        print!("  - {file_name} : ");
        match read_file(file_path.clone()) {
            Ok(content) => {
                match write_file(file_path, content) {
                    Ok(_) => (),
                    Err(e) => println!(" {e} "),
                }
            
            },
            Err(e) => println!("{e}"), 
        }
        


    }

}


// Read the file and check if there's already a sheet, return File
pub fn read_file(path: PathBuf) -> Result<String, String> {
    let mut file = match File::open(path) {
        Ok(s) => s,
        Err(e) => {
            trace!(" {e} ");
            return Err("Failed to open the file.".to_string());
        },
    };
    trace!("File has been opened");

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            trace!(" {e} ");
            return Err("Failed to read the file.".to_string());
            
        }
    }
    trace!("File has been read");

    if content.starts_with("-- Table:") {
        info!("Syntax has been found in the file that would suggest that the file has already a sheet generated.");
        return Err("No change required.".to_string());
        
    }

    Ok(content)
}

// Write the file
fn write_file(path: PathBuf, content: String) -> Result<(), String> {

    let mut file = match File::create(path) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to open the file.");
            trace!(" {e} ");
            return Err("Fatal Error".to_string())
        },
    };
    trace!("File has been opened in write-only mode.");


    let empty_metadata = Metadata::empty();
    let mut metadata = empty_metadata.generate_metadata_string();
    metadata.push_str(&content); 
    match file.write_all(metadata.as_bytes()) {
        Ok(_) => println!("Sheet has been added to the file."),
        Err(e) =>  {
            error!("Failed to write the file.");
            trace!(" {e} ");
            return Err("Fatal Error".to_string())
        }
    }

    file.sync_all().unwrap();
    Ok(())
}
