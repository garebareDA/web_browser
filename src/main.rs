extern crate gtk;
extern crate ureq;

use gtk::prelude::*;
use gtk::{Window, WindowType};
use std::fs;
use std::env;

use web_browser::html_parser::parses::parse_node;
use web_browser::html_parser::structs::Html;
use web_browser::ui::add;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut html = Html {
        html: String::new(),
        tag: Vec::new(),
    };

    let url = &args[1];
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

    if gtk::init().is_err() {
        println!("Faild to initialize GTK");
        return;
    }

    let width = 1080;
    let height = 720;

    let window = Window::new(WindowType::Toplevel);
    let scr_win = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    window.set_title("web_blowser");
    window.set_default_size(width, height);
    let mut vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);

    let node = parse_node(&mut html);
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