use std::{path::Path, fs};

use log::trace;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub filename: String,
    pub table: Option<String>,
    pub changes: Option<String>,
    pub notes: Option<String>
}

impl Metadata {

    pub fn new(path: String) -> Self {
        Self {filename: path, table : None, changes: None, notes: None}
    }

    pub fn parse_content(&mut self, path: &str) {

        // TODO: FS shouldn't be in a METADATA Method.
        let path = Path::new(path).join(&self.filename);
        trace!("Path of the file to be parsed : {:?}", path);
        let data = fs::read_to_string(path).unwrap();
        trace!("Data has been read from the file.");

        // let data = data.split("\n");
        let data: Vec<String> = data.split("\n") // Parsing per new line
            .filter(|i| i.starts_with("--"))     // We keep only lines that are SQL comments
            .map(|i| i.replace("--", "").replace("\n", "")) // We remove characters we do not need
            .collect();

        trace!("Lines of data parsed to analyse : {}", data.len());
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
        trace!("Parsing is complete.");
    }

}
