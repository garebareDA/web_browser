extern crate gtk;

use std::fs;
use web_browser::html_parser::parses::parse_node;
use web_browser::html_parser::structs::Html;
use web_browser::ui::add;

fn main() {
    let width = 1080;
    let height = 720;

    let html = fs::read_to_string("./html/test.html")
        .unwrap()
        .replace("\r\n", "")
        .to_string();

    let mut html = Html {
        html: html,
        tag: Vec::new(),
    };

    let node = parse_node(&mut html);
}