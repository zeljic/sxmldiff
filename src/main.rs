extern crate getopts;
extern crate xmltree;

mod utils;

use getopts::{Options, Matches};
use std::env;
use xmltree::Element;

use utils::{log_print, LogLevel};

use std::io;
use std::cmp::Ordering::*;

#[derive(Debug)]
struct Attr<'attr> {
    key: &'attr str,
    value: &'attr str,
}

impl<'attr> Attr<'attr> {
    fn new(data: (&'attr str, &'attr str)) -> Attr<'attr> {

        let ret_val: Attr<'attr> = Attr {
            key: data.0,
            value: data.1,
        };

        return ret_val;
    }

    fn print(&self) -> String {
        format!("{}=\"{}\"", self.key, self.value)
    }
}

#[derive(Debug)]
struct Tag<'tag> {
    el: &'tag Element,
    attrs: Vec<Attr<'tag>>,
}

impl<'tag> Tag<'tag> {
    fn new(el: &'tag Element) -> Tag<'tag> {

        let mut ret_val: Tag<'tag> = Tag {
            el: el,
            attrs: Vec::new(),
        };

        for attr in &el.attributes {
            let attr: Attr<'tag> = Attr::<'tag>::new((attr.0, attr.1));
            ret_val.attrs.push(attr);
        }

        if ret_val.attrs.len() > 0 {
            ret_val.attrs.sort_by(|x, y| if x.key > y.key {
                Greater
            } else if x.key < y.key {
                Less
            } else {
                Equal
            });
        }

        ret_val
    }

    fn print(&self) -> String {
        let mut ret_val: String = String::from("<");

        ret_val.push_str(&self.el.name);

        for attr in &self.attrs {
            ret_val.push(' ');
            ret_val.push_str(&attr.print());
        }

        ret_val.push_str(">");

        ret_val
    }
}

fn compare_nodes(el_x: &Element, el_y: &Element) {

    let mut tag_x: Tag = Tag::new(el_x);

    if el_x.children.len() > 0 {
        for child in &el_x.children {
            compare_nodes(child, child);
        }
    } else {

    }
}

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
