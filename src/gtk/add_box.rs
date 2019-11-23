extern crate gtk;

use super::super::html_parser::structs::Nodes;
use super::tag_judg;
use gtk::prelude::*;

pub fn html_judg(view:&mut gtk::ScrolledWindow, nodes: &Nodes) {
    let mut vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    if !nodes.child.is_empty() {
        node_serch(&mut vbox, &nodes.child);
    }
    view.add(&vbox);
}

fn node_serch(vbox: &mut gtk::Box, child: &Vec<Nodes>) {
    let mut vbox_child = gtk::Box::new(gtk::Orientation::Vertical, 0);
    for index in 0..child.len() {
        let tag_name = child[index].tag_name.clone();
        let text = child[index].text.clone();

        tag_judg::tags(vbox, &child[index]);
        println!("tag:{} text:{}", tag_name, text);

        if !child[index].child.is_empty() {
            node_serch(&mut vbox_child, &child[index].child);
        }
        vbox.add(&vbox_child);
    }
}
