use web_browser::html_parser::structs::Html;
use web_browser::html_parser::parses::parse_node;
use web_browser::gtk::add_box;
use std::fs;

use std::env;

fn main() {
    let html = fs::read_to_string("./html/test.html").unwrap().replace("\r\n", "").to_string();
    let mut html = Html{
        html:html,
        tag:Vec::new(),
    };

    let node = parse_node(&mut html);
}