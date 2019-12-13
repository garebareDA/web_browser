extern crate gtk;

use gtk::prelude::*;

use super::image_tag::image;
use super::list_tag::li_tag;
use super::text_tag::{a_tag, hr, label_h, textarea};
use crate::html_parser::structs::Attribute;

pub fn tag_judgment(
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
            "a" => {
                let a = a_tag(&attr, &text);
                vbox.pack_start(&a, false, false, 5);
            }
            _ => {}
        }
    }

    if text != "" {
        let last_tag = inner.last().unwrap();
        if last_tag != "li" && last_tag != "a" {
            vbox.pack_start(&label_h(font_size, markup, &text), false, false, pad);
        }
    }
}
