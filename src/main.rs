extern crate gtk;
extern crate ureq;

use gtk::prelude::*;
use gtk::{Window, WindowType};
use std::env;

use web_browser::html_parser::structs::Html;
use web_browser::ui;

fn main() {
    if gtk::init().is_err() {
        println!("Faild to initialize GTK");
        return;
    }

    let width = 1080;
    let height = 720;

    let window = Window::new(WindowType::Toplevel);
    let mut scr_win = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    window.set_title("web_blowser");
    window.set_default_size(width, height);

    let mut vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let mut html = Html {
        html: String::new(),
        tag: Vec::new(),
    };
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    ui::add::add_box(url, &mut vbox, &mut html);
    scr_win.add(&vbox);
    window.add(&scr_win);
    window.show_all();

    window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });

    gtk::main();
}