extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType};

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

    let window = Window::new(WindowType::Toplevel);
    let scr_win = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    window.set_title("web_blowser");
    window.set_default_size(width, height);
    let mut vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);

    let node = parse_node(&mut html);
    println!("{:?}", node);
    add::node_serch(&node.child, &mut vbox, &mut Vec::new());

    scr_win.add(&vbox);
    window.add(&scr_win);
    window.show_all();

    window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });

    gtk::main();
}