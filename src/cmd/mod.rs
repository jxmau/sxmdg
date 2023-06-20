use std::{path::PathBuf, process::{exit, Command}};

use log::{error, trace, info};

pub mod generate;
pub mod sheet;
pub mod new;


pub fn get_pwd() -> PathBuf {
    trace!("Fetching and verifying the pwd");
    let pwd = match std::env::current_dir() {
        Ok(pwd) => pwd,
        Err(e) => {
            println!("Fatal error.");
            error!("Impossible to fetch the current directory path.");
            trace!(" {e} ");
            exit(1);
        },
    };
    pwd
}

pub fn is_sqlx_installed() {
    info!("Checking if sqlx is installed.");
    match Command::new("sqlx").spawn() {
        Ok(_) => info!("sqlx-cli is installed."),
        Err(e) => {
            println!("sqlx-cli hasn't been found.");
            info!("sqlx-cli hasn't been found.");
            trace!(" {e} ");
        }
    }
}