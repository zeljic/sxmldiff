extern crate ansi_term;
extern crate getopts;

use std;
use std::result::Result;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use getopts::Options;

#[derive(Debug)]
pub enum UtilsErrors {
    IoOpen(String),
    IoRead(String),
}

impl std::fmt::Display for UtilsErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            UtilsErrors::IoOpen(ref path) => {
                let str: String = log(LogLevel::ERROR,
                                      format!("Unable to open file {}", path).as_str());

                write!(f, "{}", &str)
            }
            UtilsErrors::IoRead(ref path) => {
                let str: String = log(LogLevel::ERROR,
                                      format!("Unable to read file {}", path).as_str());

                write!(f, "{}", &str)
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

#[derive(Debug)]
pub enum LogLevel {
    INFO,
    WARNING,
    ERROR,
}

pub fn log(level: LogLevel, content: &str) -> String {

    let prefix = match level {
        LogLevel::INFO => ansi_term::Colour::White.paint("[INFO]"),
        LogLevel::WARNING => ansi_term::Colour::Yellow.paint("[WARNING]"),
        LogLevel::ERROR => ansi_term::Colour::Red.paint("[ERROR]"),
    };

    format!("{} {}", &prefix, content)
}

pub fn log_print(level: LogLevel, content: &str) {
    println!("{}", log(level, content));
}

pub fn repeat_char(c: char, times: usize) -> String {
    std::iter::repeat(c).take(times).collect::<String>()
}

pub fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE1 FILE2 [options]", program);
    print!("{}", opts.usage(&brief));
}