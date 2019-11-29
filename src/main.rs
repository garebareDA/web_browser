#[macro_use]
extern crate conrod;

use conrod::{widget, color, Colorable, Borderable, Sizeable, Positionable, Labelable, Widget};
use conrod::backend::glium::glium;
use conrod::glium::glutin::Event;
use conrod::backend::glium::glium::Surface;

use web_browser::html_parser::structs::Html;
use web_browser::html_parser::parses::parse_node;
use web_browser::conrod::add_box;
use std::fs;

use std::env;

widget_ids!(
    struct Ids {
        canvas,
        num_lbl,
        button,
    }
);

fn main() {

    let width = 400;
    let height = 200;

    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("web_browser")
        .with_dimensions((width, height).into());

    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();
    let ids = &mut Ids::new(ui.widget_id_generator());
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let mut events:Vec<glium::glutin::Event> = Vec::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let html = fs::read_to_string("./html/test.html").unwrap().replace("\r\n", "").to_string();
    let mut html = Html{
        html:html,
        tag:Vec::new(),
    };

    let node = parse_node(&mut html);

    'render: loop {
        events.clear();
        event_loop.poll_events(|event| { events.push(event);});

        if !events.is_empty() {
            println!("{:?}", events);
            match events[0]{
                Event::WindowEvent{event: glium::glutin::WindowEvent::CloseRequested, ..} => break 'render,
                _ => {}
            }
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
         }

    }
}