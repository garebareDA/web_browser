extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Button, Label, Window, WindowType};

use std::env;

pub fn window() {
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);

        win.set_default_size(320, 200);
        win.set_title("web blowser");
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}