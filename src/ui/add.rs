extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};
use gtk::{LabelBuilder, Justification, TextTagBuilder};

use crate::html_parser::structs::Nodes;

pub fn node_serch(child: &Vec<Nodes>, vbox:&mut gtk::Box) {
    for index in 0..child.len() {
        let tag_name = child[index].tag_name.clone();
        let text = child[index].text.clone();

        if tag_name == "head" {
            continue;
        }
        let mut vboxs = gtk::Box::new(gtk::Orientation::Vertical, 2);
        tag_judgment(&tag_name, &text, &mut vboxs);

        if !child[index].child.is_empty() {
            node_serch(&child[index].child, &mut vboxs);
        }

        vbox.add(&vboxs);
    }
}

fn tag_judgment(tag:&str, text:&str, vbox:&mut gtk::Box) {
    match tag{
        "h1" =>  vbox.pack_start(&label_h( 30000, "b", text), false, false, 5),
        "h2" =>  vbox.pack_start(&label_h( 25000, "b", text), false, false, 5),
        "h3" =>  vbox.pack_start(&label_h( 23000, "b", text), false, false, 5),
        "h4" =>  vbox.pack_start(&label_h( 20000, "b", text), false, false, 5),
        "h5" =>  vbox.pack_start(&label_h( 15000, "b", text), false, false, 10),
        "h6" =>  vbox.pack_start(&label_h( 10000, "b", text), false, false, 10),
        "p" => vbox.pack_start(&label_h( 15000, "span", text), false, false, 5),
        _ => {}
    }
}

fn label_h(size:u32, use_tag:&str, text:&str) -> gtk::Label {
    let label = gtk::Label::new(None);
    let markup = format!("<{}><span size='{}'>{}</span></{}>",use_tag, size, text, use_tag);
    label.set_justify(Justification::Left);
    label.set_halign(gtk::Align::Start);
    label.set_markup(&markup);
    label.set_margin_start(10);

    return label;
}