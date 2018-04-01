#[macro_use]
extern crate clap;
extern crate colored;
extern crate xmltree;

mod utils;

use std::cmp::{Ord, Ordering};

use colored::*;
use xmltree::Element;

use utils::{log_print, LogLevel};

use clap::{App, Arg};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Attr<'attr> {
    key: &'attr str,
    value: &'attr str,
}

impl<'attr> Attr<'attr> {
    fn new(data: (&'attr str, &'attr str)) -> Attr<'attr> {
        Attr {
            key: data.0,
            value: data.1,
        }
    }

    fn print(&self) -> String {
        format!("{}=\"{}\"", self.key, self.value)
    }
}

impl<'attr> Ord for Attr<'attr> {
    fn cmp(&self, rhs: &Attr<'attr>) -> Ordering {
        self.key.cmp(rhs.key)
    }
}

#[derive(Debug)]
struct Tag<'tag> {
    el: &'tag Element,
    attrs: Vec<Attr<'tag>>,
}

impl<'tag> PartialEq for Tag<'tag> {
    fn eq(&self, rhs: &Tag<'tag>) -> bool {
        match self.el.text {
            Some(_) => self.el.text == rhs.el.text,
            None => self.el.name == rhs.el.name && self.attrs == rhs.attrs,
        }
    }
}

impl<'tag> Tag<'tag> {
    fn new(el: &'tag Element) -> Tag<'tag> {
        let mut ret_val: Tag<'tag> = Tag {
            el,
            attrs: Vec::new(),
        };

        for attr in &el.attributes {
            let attr: Attr<'tag> = Attr::<'tag>::new((attr.0, attr.1));
            ret_val.attrs.push(attr);
        }

        ret_val.attrs.sort();

        ret_val
    }

    fn print(&self, indent: &usize) -> String {
        let mut ret_val: String = " ".repeat(*indent);
        ret_val.push('<');

        ret_val.push_str(&self.el.name);

        for attr in &self.attrs {
            ret_val.push(' ');
            ret_val.push_str(&attr.print());
        }

        ret_val.push_str(">");

        if let Some(ref text) = self.el.text {
            ret_val.push_str(text);
            ret_val.push_str(&format!("</{}>", self.el.name));
        }

        ret_val
    }

    fn print_end(&self, indent: &usize) -> Option<String> {
        if !self.has_text() {
            Some(format!("{}</{}>", " ".repeat(*indent), self.el.name))
        } else {
            None
        }
    }

    fn has_text(&self) -> bool {
        self.el.text.is_some()
    }

    fn print_diff(&self, indent: &usize) -> String {
        format!(
            "{}{}\n{}",
            "|".green(),
            &self.print(&if *indent > 0 { *indent - 1 } else { 0 }),
            "|".red()
        )
    }
}

fn compare_nodes<'cn>(tag_x: &Tag<'cn>, tag_y: &Tag<'cn>, indent: &'cn mut usize) {
    let start_tag = tag_x.print(indent);
    let end_tag = tag_x.print_end(indent);

    *indent += 2;

    println!("{}", start_tag);

    for child_x in &tag_x.el.children {
        let tag_child_x: Tag = Tag::new(child_x);
        let mut found: bool = false;

        for child_y in &tag_y.el.children {
            let tag_child_y: Tag = Tag::new(child_y);

            if tag_child_x == tag_child_y {
                found = true;

                compare_nodes(&tag_child_x, &tag_child_y, indent);

                break;
            }
        }

        if !found {
            println!("{}", tag_child_x.print_diff(indent));
        }
    }

    if let Some(v) = end_tag {
        println!("{}", v)
    }

    *indent -= 2;
}

fn main() {
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("FILE1").required(true).index(1))
        .arg(Arg::with_name("FILE2").required(true).index(2))
        .get_matches();

    let file1_path: String = value_t_or_exit!(args.value_of("FILE1"), String);
    let file2_path: String = value_t_or_exit!(args.value_of("FILE2"), String);

    let content_x: String = match utils::read_file_content(&file1_path) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let content_y: String = match utils::read_file_content(&file2_path) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let el_x: Element = match Element::parse(content_x.as_bytes()) {
        Ok(el) => el,
        Err(_) => {
            log_print(&LogLevel::ERROR, "Unable to parse XML document.");
            return;
        }
    };

    let el_y: Element = match Element::parse(content_y.as_bytes()) {
        Ok(el) => el,
        Err(_) => {
            log_print(&LogLevel::ERROR, "Unable to parse XML document.");
            return;
        }
    };

    let mut indent: usize = 0;
    let tag_x: Tag = Tag::new(&el_x);
    let tag_y: Tag = Tag::new(&el_y);

    if tag_x == tag_y {
        compare_nodes(&tag_x, &tag_y, &mut indent);
    } else {
        tag_x.print_diff(&indent);
    }
}
