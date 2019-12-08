extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};
use gtk::{LabelBuilder, Justification, TextTagBuilder};

use std::fs;
use web_browser::html_parser::parses::parse_node;
use web_browser::html_parser::structs::Html;
use web_browser::ui::add;

fn main() {

    if gtk::init().is_err() {
        println!("Faild to initialize GTK");
        return;
    }


    let width = 1080;
    let height = 720;

    let html = fs::read_to_string("./html/test.html")
        .unwrap()
        .replace("\r\n", "")
        .to_string();

    let mut html = Html {
        html: html,
        tag: Vec::new(),
    };

    let node = parse_node(&mut html);

    let window = Window::new(WindowType::Toplevel);
    window.set_title("web_blowser");
    window.set_default_size(width, height);

    //labelのtextサイズがmarkupで size

    let buffer = gtk::Label::new(Some("mario"));
    buffer.set_markup("<span size='100000'>big text</span>");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);

    vbox.add(&buffer);
    window.add(&vbox);

    window.show_all();

    window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });

    gtk::main();
}