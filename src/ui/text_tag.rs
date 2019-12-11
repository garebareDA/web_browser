extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Label, WidgetExt};
use gtk::Justification;

pub fn label_h(size:u32, use_tag:&str, text:&str) -> gtk::Label {
    let label = Label::new(None);
    let markup = format!("<{}><span size='{}'>{}</span></{}>",use_tag, size, text, use_tag);
    label.set_justify(Justification::Left);
    label.set_halign(gtk::Align::Start);
    label.set_markup(&markup);
    label.set_margin_start(10);

    return label;
}

pub fn hr() -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    vbox.override_background_color(gtk::StateFlags::NORMAL, Some(&gdk::RGBA::black()));
    vbox.set_property_height_request(1);
    vbox.set_margin_end(10);
    vbox.set_margin_start(10);
    vbox.set_opacity(0.5);
    return vbox
}