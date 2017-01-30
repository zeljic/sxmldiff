extern crate ansi_term;

use std;
use std::result::Result;
use std::fs::OpenOptions;
use std::io::prelude::Read;

#[derive(Debug)]
pub enum UtilsErrors {
    IoOpen(String),
    IoRead(String),
}

impl std::fmt::Display for UtilsErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            UtilsErrors::IoOpen(ref path) => {
                write!(f,
                       "{} Unable to open file {}",
                       ansi_term::Colour::Red.paint("[ERROR]"),
                       path)
            }
            UtilsErrors::IoRead(ref path) => {
                write!(f,
                       "{} Unable to read file content {}",
                       ansi_term::Colour::Red.paint("[ERROR]"),
                       path)
            }
        }
    }
}

pub fn read_file_content(path: &str) -> Result<String, UtilsErrors> {
    match OpenOptions::new().read(true).open(path) {
        Ok(mut file) => {
            let mut content: String = String::new();

            match file.read_to_string(&mut content) {
                Ok(_) => Ok(content),
                Err(_) => Err(UtilsErrors::IoRead(path.into())),
            }
        }
        Err(_) => Err(UtilsErrors::IoOpen(path.into())),
    }
}
