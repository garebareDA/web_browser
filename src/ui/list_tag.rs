extern crate gtk;

use gtk::prelude::*;
use gtk::{Box, Label, Orientation, WidgetExt};

use super::text_tag;

pub fn li_tag(mark: &str, label: Label, mut font_size: u32, margin: i32, ol: bool) -> Box{
    let hbox = Box::new(Orientation::Horizontal, 2);

    if !ol {
        if 10000 < font_size && font_size < 25000 {
            font_size -= 10000;
        }else if 25000 < font_size{
            font_size -= 20000;
        }else{
            font_size -= 6000;
        }
    }

    let mark = text_tag::label_h(font_size, "span", mark);
    hbox.add(&mark);
    hbox.add(&label);
    hbox.set_margin_start((margin - 1) * 50);
    hbox
}