extern crate gtk;
extern crate ureq;

use gtk::prelude::*;
use gtk::{Button, Label, Window, WidgetExt};
use gtk::Justification;
use std::io::{Read, Write};
use std::fs::File;

use crate::html_parser::structs::{Nodes, Attribute};
use super::image_tag::image;
use super::text_tag::{label_h, hr};

pub fn node_serch(child: &Vec<Nodes>, vbox:&mut gtk::Box, inner: &mut Vec<String>) {
    for index in 0..child.len() {
        let tag_name = child[index].tag_name.clone();
        let text = child[index].text.clone();
        let attr = &child[index].attributes;
        let mut vboxs = gtk::Box::new(gtk::Orientation::Vertical, 2);

        if &tag_name == "head" {
            continue;
        }

        if &tag_name != "html"{
            inner.push(tag_name.clone());
        }

        tag_judgment(&text, &mut vboxs, attr, inner);

        if !child[index].child.is_empty() {
            node_serch(&child[index].child, &mut vboxs, inner);
        }

        if !inner.is_empty(){
            inner.remove(inner.len() - 1);
        }

        vbox.add(&vboxs);
    }
}

fn tag_judgment(text:&str, vbox:&mut gtk::Box, attr: &std::vec::Vec<Attribute>, inner: &mut Vec<String>) {
    let mut font_size = 15000;
    let mut markup = "span";
    let mut pad = 5;

    for index in 0..inner.len(){
        let tag : &str = &inner[index];
        match tag {
            "h1" =>  {font_size = 30000; markup = "b"; pad = 5;}
            "h2" =>  {font_size = 25000; markup = "b"; pad = 5;}
            "h3" =>  {font_size = 23000; markup = "b"; pad = 5;}
            "h4" =>  {font_size = 20000; markup = "b"; pad = 5;}
            "h5" =>  {font_size = 15000; markup = "b"; pad = 10;}
            "h6" =>  {font_size = 10000; markup = "b"; pad = 10;}
            "p" =>  {font_size = 15000; markup = "span"; pad = 5;}
            "hr" => vbox.pack_start(&hr(), false, false, 15),
            "img" => vbox.pack_start(&image(attr), false, false, 10),
            _ => {}
        }
    }

    let text = text.trim();
    if text != ""{
        println!("{}",text);
        vbox.pack_start(&label_h(font_size, markup, text), false, false, pad);
    }
}