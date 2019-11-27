#[macro_use]
extern crate conrod;


use conrod::{widget, color, Colorable, Borderable, Sizeable, Positionable, Labelable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;

use web_browser::html_parser::structs::Html;
use web_browser::html_parser::parses::parse_node;
use web_browser::conrod::add_box;
use std::fs;

use std::env;

fn main() {
    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("web_browser")
        .with_dimensions((400, 200).into());

    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let mut events = Vec::new();
    'render: loop {
        events.clear();
        event_loop.poll_events(|event| { events.push(event);});

        if events.is_empty() {
            event_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            })
        }
    }

    let html = fs::read_to_string("./html/test.html").unwrap().replace("\r\n", "").to_string();
    let mut html = Html{
        html:html,
        tag:Vec::new(),
    };

    let node = parse_node(&mut html);
}