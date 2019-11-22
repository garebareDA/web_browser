use web_browser::html_parser::structs::Html;
use web_browser::html_parser::parses::parse_node;
use web_browser::gtk::window::window;
use std::fs;

fn main() {
    let html = "<html><body>hello world<p aaaa=\"bbbbb\" ccccc=\"ddddd\">aaaaa<p aaaa=\"bbbbbb\">dddddd</p><p>ffff<div>eeeeeee</div></p></p><p>bbbbbbbb<p>hhhhhh</p></p><p>cccc<p>ggggggg</p></p></body></html>".to_string();
    let html = fs::read_to_string("./html/test.html").unwrap().replace("\r\n", "").to_string();
    let mut html = Html{
        html:html,
        tag:Vec::new(),
    };

    let node = parse_node(&mut html);
    window();
    println!("{:?}", node);
}