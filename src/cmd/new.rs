use log::{info, trace, error};
use core::time;
use std::{process::{exit, Command}, fmt::format, fs::DirEntry, thread};

use crate::cmd::get_pwd;

use super::is_sqlx_installed;

pub fn new(table: Option<String>) {

    let table = match table {
        Some(table) => table,
        None => {
            eprintln!("No Table name has been provided");
            exit(1);
        }
    };

    is_sqlx_installed();

    let command = format!("sqlx migrate add {table}");
    trace!("Command about to be executed : {command} ");
    match Command::new("sqlx").arg("migrate").args(["add", &table]).spawn() {
        Ok(s) => {
            info!("sqlx migrate add command has succeeded");
            trace!(" {s:?} ");
        },
        Err(e) => {
            eprintln!("Fatal Error");
            info!("sqlx migrate add command has failed");
            trace!(" {e} ");
        }
    }

    trace!("Program about to retrieve the file created.");
    let mut pwd = get_pwd();
    pwd.push("migrations");
    trace!("pwd : {pwd:?}");

    let files = match pwd.read_dir(){
        Ok(files) => files,
        Err(e) => {
            println!("Fatal Error.");
            error!("Failed to read the entries of the migrations directory");
            trace!("Error : {e} ");
            exit(1);
        }
    };

    let mut table_files : Vec<String> = Vec::new();
    let suffix = format!("{table}.sql");
    for f in files.into_iter() {
        if let Ok(name) = f {
            if let Ok(filename) = name.file_name().into_string() {
                if filename.ends_with(&suffix) {
                    table_files.push(filename)
                }
            }
        }
    };

    thread::sleep(time::Duration::from_secs(1));

    table_files.sort();
    trace!("Files found ({}) : {table_files:?}", table_files.len())

}