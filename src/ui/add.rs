extern crate gtk;

use gtk::prelude::*;

use super::image_tag::image;
use super::list_tag::li_tag;
use super::text_tag::{hr, label_h, textarea};
use crate::html_parser::structs::{Attribute, Nodes};

pub fn node_serch(child: &Vec<Nodes>, vbox: &mut gtk::Box, inner: &mut Vec<String>) {
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

        tag_judgment(&text, &mut vboxs, attr, inner, li_count);

        if !child[index].child.is_empty() {
            node_serch(&child[index].child, &mut vboxs, inner);
        }

        if !inner.is_empty() {
            inner.remove(inner.len() - 1);
        }

        vbox.add(&vboxs);
    }
}

fn tag_judgment(
    text: &str,
    vbox: &mut gtk::Box,
    attr: &std::vec::Vec<Attribute>,
    inner: &mut Vec<String>,
    li_count: usize,
) {
    let mut font_size = 15000;
    let mut markup = "span";
    let mut pad = 5;
    let text = text.trim().to_string();
    let mut li_mark = "● ".to_string();
    let mut ul_count = 0;
    let mut ol = false;

    for index in 0..inner.len() {
        let tag: &str = &inner[index];
        match tag {
            "h1" => {
                font_size = 30000;
                markup = "b";
                pad = 5;
            }
            "h2" => {
                font_size = 25000;
                markup = "b";
                pad = 5;
            }
            "h3" => {
                font_size = 23000;
                markup = "b";
                pad = 5;
            }
            "h4" => {
                font_size = 20000;
                markup = "b";
                pad = 5;
            }
            "h5" => {
                font_size = 15000;
                markup = "b";
                pad = 10;
            }
            "h6" => {
                font_size = 10000;
                markup = "b";
                pad = 10;
            }
            "p" => {
                font_size = 15000;
                markup = "span";
                pad = 5;
            }
            "hr" => vbox.pack_start(&hr(), false, false, 15),
            "img" => vbox.pack_start(&image(attr), false, false, 10),
            "ul" => {
                ul_count += 1;
                if ul_count == 1 {
                    li_mark = "●   ".to_string();
                } else if ul_count == 2 {
                    li_mark = "◯   ".to_string();
                } else {
                    li_mark = "■   ".to_string();
                }
                ol = false;
            }
            "ol" => {
                ul_count += 1;
                li_mark = format!("{}. ", li_count);
                ol = true
            }
            "li" => vbox.pack_start(
                &li_tag(
                    &li_mark,
                    label_h(font_size, markup, &text),
                    font_size,
                    ul_count,
                    ol,
                ),
                false,
                false,
                0,
            ),
            "textarea" => vbox.pack_start(&textarea(&attr), false, false, 5),
            _ => {}
        }
    }

    if text != "" {
        if inner.last().unwrap() != "li" {
            println!("{}", text);
            vbox.pack_start(&label_h(font_size, markup, &text), false, false, pad);
        }
    }
}
