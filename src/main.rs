extern crate getopts;
extern crate xmltree;

mod utils;

use getopts::{Options, Matches};
use std::env;
use xmltree::Element;

use utils::{log_print, LogLevel};

use std::cmp::{Ord, Ordering};

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
            el: el,
            attrs: Vec::new(),
        };

        for attr in &el.attributes {
            let attr: Attr<'tag> = Attr::<'tag>::new((attr.0, attr.1));
            ret_val.attrs.push(attr);
        }

        ret_val.attrs.sort();

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

        if let Some(ref text) = self.el.text {
            ret_val.push_str(text);
            ret_val.push_str(&format!("</{}>", self.el.name));
        }

        ret_val
    }

    fn print_end(&self) -> Option<String> {
        if self.el.text.is_none() {
            Some(format!("</{}>", self.el.name))
        } else {
            None
        }
    }
}

fn print_diff(tag_x: &Tag, tag_y: &Tag, indent: &mut usize) -> String {
    "".into()
}

fn compare_nodes<'cn>(tag_x: &Tag<'cn>, tag_y: &Tag<'cn>, indent: &mut usize) {

    if tag_x == tag_y {
        let indent_str = utils::repeat_char(' ', *indent);
        let start_tag = tag_x.print();
        let end_tag = tag_x.print_end();

        *indent += 2;

        println!("{}{}", indent_str, start_tag);

        for child_x in &tag_x.el.children {
            let tag_child_x: Tag = Tag::new(child_x);

            match tag_y.el.children.iter().find(|e| e.name == child_x.name) {
                Some(el) => compare_nodes(&tag_child_x, &Tag::new(el), indent),
                None => {}
            }
        }

        if let Some(v) = end_tag {
            println!("{}{}", indent_str, v)
        }

        *indent -= 2;

    } else {
        print_diff(tag_x, tag_y, indent);
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

    let mut indent: usize = 0;

    compare_nodes(&Tag::new(&el_x), &Tag::new(&el_y), &mut indent);
}
