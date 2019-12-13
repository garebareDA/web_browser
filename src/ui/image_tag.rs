extern crate gtk;
extern crate ureq;

use gtk::prelude::*;
use gtk::Image;
use std::io::{Read, Write};
use std::fs::File;
use crate::html_parser::structs::Attribute;

pub fn image(attr: &std::vec::Vec<Attribute>) -> gtk::Image {
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