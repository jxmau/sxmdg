use std::{fs::{File}, collections::{ HashMap}, ffi::OsString, io::Write};

use log::{error, trace, info};

use crate::metadata::Metadata;


pub fn generate(path: Option<String>) {
    info!("Commande chosen : generate");
    trace!("Getting the current directory path (pwd).");
    let mut pwd = match std::env::current_dir() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Impossible to get the current path.");
            error!("Error : {e}");
            std::process::exit(1);
        }
    };
    if let Some(s) = path {
        pwd.push(s);
        trace!("Path supplied in the cli.");
    }
    trace!("Absolute path is : {pwd:?}");
    // Cloned variable to be used later when writing the file.
    let mut pwdbis = pwd.clone();

    pwd.push("migrations");
    let path = pwd.to_str().to_owned().unwrap(); // Why does this var exists?
    
    trace!("Reading entries in the migrations directory");
    let files = match pwd.read_dir() {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to read the entries of the migrations directory.");
            trace!("Error : {e} ");
            std::process::exit(1);
        }
    };


    let mut tables : HashMap<String, Vec<Metadata>> = HashMap::new();

    // TODO: To refractor later, but seemingly really likely to break something
    info!("Starting to read files acquired.");
    for file in files {
        if let Ok(file_read) = file {
            match file_read.file_name().into_string() {
                Ok(s) => {
                    if s.ends_with(".sql") {
                        
                        let mut metadata = Metadata::new(s);
                        metadata.parse_content(path);
                        let table = match metadata.table.clone() {
                            Some(s) if s.eq("") => "Table Unknown".to_string(),
                            Some(s) => s,
                            None => "Table Unknown".to_string(),
                        };

                        match tables.get_mut(&table) {
                            Some(metadatas) => metadatas.push(metadata),
                            None => {tables.insert(table, vec![metadata]);},
                        }
                    }

                },
                Err(e) => {
                    error!("Failed to convert OsString to String. Ignoring it.");
                    trace!("OsString that failed : {e:?}");
                },
            }
        }
    }
    
    let s = generate_md(tables);
    write_file(s, pwdbis.as_mut_os_string());
    info!("End of the Generate command.");
}

fn generate_md(tables: HashMap<String, Vec<Metadata>>) -> String {
    info!("Generating the Database.MD");
    trace!("Table(s) found : {}", tables.len());

    let mut file_string_builder = String::new();

    for (t, metadatas) in tables {
        trace!("Building table for Table : {t} -- Entries parsed : {}", metadatas.len());
        let mut section_string_builder = String::from(format!("## {t} \n \n "));
        section_string_builder.push_str("Filename | Version | Changes | Notes | \n");
        section_string_builder.push_str(" --- | --- | --- | --- | \n");
        for m in metadatas {

            let filename = m.filename.clone();
            let table_row = format!(" [{}](/{}) | {} | {} | {} \n", filename, filename, m.version.clone().unwrap_or(format!("Unspecified")), m.changes.clone().unwrap_or("None".to_string()), m.notes.clone().unwrap_or("None".to_string()));
            section_string_builder.push_str(&table_row);
            
        }
        section_string_builder.push_str("\n\n");
        file_string_builder.push_str(&section_string_builder);
    }
    info!("Database.MD has been generated!");
    file_string_builder

}


fn write_file(s: String, pwd: &mut OsString) {
    info!("Writing Database.MD");
    pwd.push("/database.MD");
    trace!("Path of the file to be written : {pwd:?}");

    let mut file = match File::create(pwd) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to create a File.");
            trace!(" Error : {e} ");
            std::process::exit(1);
        },
    };

    match file.write_all(s.as_bytes()) {
        Ok(_) => {
            info!("File has been generated and saved!")
        },
        Err(e) => {
            error!("Failed to save the file.");
            trace!(" {e} ");
        },
    }
}