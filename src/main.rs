extern crate getopts;
extern crate xmltree;

mod utils;

use getopts::{Options, Matches};
use std::env;
use xmltree::Element;

use utils::{log_print, LogLevel};

fn compare_nodes(el_x: &Element, el_y: &Element) {}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();

    let mut opts: Options = Options::new();
    opts.optopt("h", "help", "Show help page", "");
    //opts.optopt("f", "output-file", "Save result to file", "");
    //opts.optopt("v", "version", "Show version", "");

    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(v) => v,
        Err(_) => {
            utils::print_usage(&program, &opts);
            return;
        }
    };

    if matches.opt_present("h") {
        utils::print_usage(&program, &opts);
        return;
    }

    if matches.free.len() < 2 {
        utils::print_usage(&program, &opts);
        return;
    }

    let content_x: String = match utils::read_file_content(&matches.free[0]) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let content_y: String = match utils::read_file_content(&matches.free[1]) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let el_x: Element = match Element::parse(content_x.as_bytes()) {
        Ok(el) => el,
        Err(_) => {
            log_print(LogLevel::ERROR, "Unable to parse XML document.");
            return;
        }
    };

    let el_y: Element = match Element::parse(content_y.as_bytes()) {
        Ok(el) => el,
        Err(_) => {
            log_print(LogLevel::ERROR, "Unable to parse XML document.");
            return;
        }
    };

    compare_nodes(&el_x, &el_y);
}
