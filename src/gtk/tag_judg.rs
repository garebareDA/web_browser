extern crate gtk;

use super::super::html_parser::structs::Nodes;
use gtk::prelude::*;

//TODO タグを実装する
fn tags(vbox:&mut gtk::Box, node: &mut Nodes) {
    match node.tag_name {
        "p"=> {}
        _ => {}
    }
}