extern crate gtk;
extern crate gio;

use web_browser::html_parser::structs::Html;
use web_browser::html_parser::parses::parse_node;
use web_browser::gtk::add_box;
use std::fs;

use gtk::prelude::*;
use gio::prelude::*;

use std::env;

fn main() {
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(|app| {

        let win = gtk::ApplicationWindow::new(app);

        win.set_default_size(800, 600);
        win.set_title("web blowser");

        let mut scr_view = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);

        let html = fs::read_to_string("./html/test.html").unwrap().replace("\r\n", "").to_string();
        let mut html = Html{
            html:html,
            tag:Vec::new(),
        };

        let mut node = parse_node(&mut html);

        add_box::html_judg(&mut scr_view, &mut node);

        win.add(&scr_view);
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}