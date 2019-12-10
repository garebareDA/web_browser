extern crate gtk;
extern crate ureq;

use gtk::prelude::*;
use gtk::{Button, Label, Window, WidgetExt, Image};
use gtk::Justification;
use std::io::{Read, Write};
use std::fs::File;

use crate::html_parser::structs::{Nodes, Attribute};

pub fn node_serch(child: &Vec<Nodes>, vbox:&mut gtk::Box) {
    for index in 0..child.len() {
        let tag_name = child[index].tag_name.clone();
        let text = child[index].text.clone();
        let attr = &child[index].attributes;

        if tag_name == "head" {
            continue;
        }

        let mut vboxs = gtk::Box::new(gtk::Orientation::Vertical, 2);
        tag_judgment(&tag_name, &text, &mut vboxs, attr);

        if !child[index].child.is_empty() {
            node_serch(&child[index].child, &mut vboxs);
        }

        vbox.add(&vboxs);
    }
}

fn tag_judgment(tag:&str, text:&str, vbox:&mut gtk::Box, attr: &std::vec::Vec<Attribute>) {
    match tag{
        "h1" =>  vbox.pack_start(&label_h( 30000, "b", text), false, false, 5),
        "h2" =>  vbox.pack_start(&label_h( 25000, "b", text), false, false, 5),
        "h3" =>  vbox.pack_start(&label_h( 23000, "b", text), false, false, 5),
        "h4" =>  vbox.pack_start(&label_h( 20000, "b", text), false, false, 5),
        "h5" =>  vbox.pack_start(&label_h( 15000, "b", text), false, false, 10),
        "h6" =>  vbox.pack_start(&label_h( 10000, "b", text), false, false, 10),
        "p" => vbox.pack_start(&label_h( 15000, "span", text), false, false, 5),
        "img" => vbox.pack_start(&image(attr), false, false, 10),
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

fn image(attr: &std::vec::Vec<Attribute>) -> gtk::Image {
    let image = Image::new();
    for index in 0..attr.len() {
        let name =  attr[index].name.clone();

        if &name == "src"{
            let url = &attr[index].contents;
            let split_url = url.split("/");
            let split_url: Vec<&str> = split_url.collect();

            if split_url[0] == "http:" || split_url[0] == "https:" {
                let resp = ureq::get(url).call();
                let mut buf = {
                     let len = resp.header("Content-Length").unwrap()
                        .parse::<usize>().unwrap();
                    vec![ 0; len ]
                };
                let mut resp = resp.into_reader();

                let mut file_name = split_url[split_url.len() - 1].to_string();

                let file_split = file_name.split(".");
                let file_split:Vec<&str> = file_split.collect();
                let n = file_split[file_split.len() - 1];

                match n {
                    "png" => {},
                    "gif" => {},
                    "webm" => {},
                    "jpg" => {},
                    _ => file_name = n.to_string() + ".png"
                }

                let path = format!("img/{}", file_name);
                let mut image_file = File::create(&path).unwrap();

                resp.read_exact(&mut buf);
                image_file.write_all(&mut buf);

                let image = Image::new_from_file(path);
                image.set_halign(gtk::Align::Start);
                image.set_margin_start(10);
                return image;
            }else if split_url[0] == "."{
                let image = Image::new_from_file(url);
                image.set_halign(gtk::Align::Start);
                image.set_margin_start(10);
                return image;
            }
        }
    }
    image
}