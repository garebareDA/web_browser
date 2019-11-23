extern crate gtk;

use super::super::html_parser::structs::Nodes;
use gtk::{Button, Label, Window, WindowType};
use gtk::prelude::*;

//TODO タグを実装する
pub fn tags(vbox:&mut gtk::Box, node: &Nodes) {
    let tag:&str = &node.tag_name;

    match tag{
        "p" => {p_tag(vbox, node)},
        _ => {},
    }
}

fn p_tag(vbox: &mut gtk::Box, node: &Nodes) {
    let Label = gtk::Label::new(Some(&node.text));
    vbox.add(&Label);
}