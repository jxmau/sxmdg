use std::{fs::{DirEntry, File, self}, collections::HashSet, io::{Read, Write}, ffi::{OsString, OsStr}, path::Path, fmt::format};

use log::{trace, error};
fn main() {
    env_logger::init();

    let mut args = std::env::args();
    trace!("Args given : {args:?} ");

    // Command is the 2nd item on the Iterator
    match args.nth(1) {
        Some(s) => match &s as &str {
            "generate" => generate(args.nth(2)),
            "sheet" => todo!(),
            _ => println!("Invalid Command.")
        },
        None => println!("No command provided.")
    }

}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub filename: OsString,
    pub table: Option<String>,
    pub changes: Option<String>,
    pub notes: Option<String>
}

impl Metadata {

    pub fn new(path: OsString) -> Self {
        Self {filename: path, table : None, changes: None, notes: None}
    }

    pub fn parse_file(&mut self, path: &str) {
        let path = Path::new(path).join(&self.filename);
        trace!("Filepath : {:?}", path);
        let data = fs::read_to_string(path).unwrap();
        println!("{data} ");
        let data = data.split("\n");
        println!("{data:?} ");
        let data: Vec<String> = data.filter(|i| i.starts_with("--")).map(|i| i.replace("--", "").replace("\n", "")).collect();
        println!("{data:?} ");
        for d in data {
            let d = d.split_once(":");
            if let Some((k, v)) = d {
                let v = Some(v.trim().to_owned());
                match k.trim() {
                    "Table" => self.table = v,
                    "Change" => self.changes = v,
                    "Notes" => self.notes = v,
                    _ => (),
                }
            }
        }
        println!("{self:?}");
    }

}

pub fn generate(path: Option<String>) {
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
    }
    let mut pwdbis = pwd.clone();
    pwd.push("migrations");
    let path = pwd.to_str().to_owned().unwrap();
    trace!("{pwd:?}");
    let files = pwd.read_dir().expect("read_dir failed");
    let mut sql_files: Vec<DirEntry> = Vec::new();
    let mut tables : HashSet<String> = HashSet::new();
    let mut list_metadata : Vec<Metadata> = Vec::new();

    for i in files {
        if let Ok(s) = i {
            if s.file_name().into_string().unwrap().ends_with(".sql") {
                
                let mut metadata = Metadata::new(s.file_name());
                metadata.parse_file(path);
                tables.insert(metadata.table.clone().unwrap_or_else(|| "Unspecified".to_string() ));
                list_metadata.push(metadata);
            }
        }
    }
    
    let s = generate_md(tables, &list_metadata);
    write_file(s, pwdbis.as_mut_os_string());

}

pub fn generate_md(tables: HashSet<String>, list_metadata: &Vec<Metadata> ) -> String {
    let mut mega_bob = String::new();
    for t in tables {
        
        let mut bob = String::from(format!("## {t} \n \n "));
        bob.push_str("Filename | Changes | Notes | \n");
        bob.push_str(" --- | --- | --- | \n");
        for m in list_metadata {
            let filename = m.filename.to_str().unwrap();
            let line = format!(" [{}](/{}) | {} | {} \n", filename, filename, m.changes.clone().unwrap_or("None".to_string()), m.notes.clone().unwrap_or("None".to_string()));
            bob.push_str(&line)
        }

        mega_bob.push_str(&bob);
    }

    mega_bob
}


pub fn write_file(s: String, pwd: &mut OsString) {
    pwd.push("/database.MD");
    // let path = Path::new(&path);
    println!("{pwd:?}");
    let mut file = File::create(pwd).unwrap();
    file.write_all(s.as_bytes()).unwrap();
}