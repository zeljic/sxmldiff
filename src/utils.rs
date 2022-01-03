#![allow(dead_code)]
extern crate colored;

use colored::*;

use std::{self, fs::OpenOptions, io::prelude::Read, result::Result};

#[derive(Debug)]
pub enum UtilsErrors {
	IoOpen(String),
	IoRead(String),
}

impl std::fmt::Display for UtilsErrors {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			UtilsErrors::IoOpen(ref path) => write!(f, "{}", log(&LogLevel::Error, &format!("Unable to open file {}", path))),
			UtilsErrors::IoRead(ref path) => write!(f, "{}", log(&LogLevel::Error, &format!("Unable to read file {}", path))),
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
	Info,
	Warning,
	Error,
}

pub fn log(level: &LogLevel, content: &str) -> String {
	let prefix: ColoredString = match *level {
		LogLevel::Info => "[INFO]".white(),
		LogLevel::Warning => "[WARNING]".yellow(),
		LogLevel::Error => "[ERROR]".red(),
	};

	format!("{} {}", &prefix.to_string().as_str(), content)
}

pub fn log_print(level: &LogLevel, content: &str) {
	println!("{}", log(level, content));
}
