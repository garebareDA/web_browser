extern crate gtk;

use super::super::html_parser::structs::Nodes;
use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};

pub fn html_judg(view:&mut gtk::ScrolledWindow, nodes: &mut Nodes) {
    if !nodes.child.is_empty() {
        node_serch(view, &mut nodes.child);
    }
}

fn node_serch(view:&mut gtk::ScrolledWindow, child: &mut Vec<Nodes>) {
    for index in 0..child.len() {
        let tag_name = child[index].tag_name.clone();
        let text = child[index].text.clone();

        println!("tag:{} text:{}", tag_name, text);

        if !child[index].child.is_empty() {
            node_serch(view, &mut child[index].child);
        }

    }
}

