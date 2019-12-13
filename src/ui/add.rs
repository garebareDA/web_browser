extern crate gtk;

use gtk::prelude::*;
use gtk::Box;
use std::fs;
use crate::html_parser::parses::parse_node;

use super::tag;
use crate::html_parser::structs::{Nodes, Html};

pub fn node_serch(child: &Vec<Nodes>, vbox: &mut gtk::Box, inner: &mut Vec<String>,) {
    for index in 0..child.len() {
        let tag_name = child[index].tag_name.clone();
        let text = child[index].text.clone();
        let attr = &child[index].attributes;
        let mut vboxs = gtk::Box::new(gtk::Orientation::Vertical, 2);
        let mut li_count = 0;

        if &tag_name == "head" {
            continue;
        }

        if &tag_name != "html" {
            inner.push(tag_name.clone());
        }

        if &tag_name == "li" {
            li_count += 1;
        }

        tag::tag_judgment(&text, &mut vboxs, attr, inner, li_count);

        if !child[index].child.is_empty() {
            node_serch(&child[index].child, &mut vboxs, inner);
        }

        if !inner.is_empty() {
            inner.remove(inner.len() - 1);
        }

        vbox.add(&vboxs);
    }
}

pub fn add_box(url: &str, vbox: &mut Box, html:&mut Html) {
    let split_url = url.split("/");
    let split_url: Vec<&str> = split_url.collect();
        if split_url[0] == "http:" || split_url[0] == "https:" {
            let resp = ureq::get(url).call();
            if resp.ok() {
            let web_html = resp.into_string().unwrap().replace("\r\n", "");
            html.html = web_html;
        }
    }else{
        let web_html = fs::read_to_string(url)
        .unwrap()
        .replace("\r\n", "")
        .to_string();

        html.html = web_html;
    }
    let node = parse_node(html);
    node_serch(&node.child, vbox, &mut Vec::new());
}