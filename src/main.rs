#[macro_use]
extern crate conrod;
extern crate find_folder;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use conrod::glium::glutin::Event;
use conrod::{color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

use std::fs;
use web_browser::ui::add_box;
use web_browser::html_parser::parses::parse_node;
use web_browser::html_parser::structs::Html;

widget_ids! {
    struct Ids {
        master,
        header,
        body,
        left_column,
        middle_column,
        right_column,
        footer,
        footer_scrollbar,
        floating_a,
        floating_b,
        tabs,
        tab_foo,
        tab_bar,
        tab_baz,

        title,
        subtitle,
        top_left,
        bottom_right,
        foo_label,
        bar_label,
        baz_label,
        button_matrix,
        bing,
        bong,
    }
}

fn main() {
    let width = 1080;
    let height = 720;

    let mut event_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("web_browser")
        .with_dimensions((width, height).into());

    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("MSGOTHIC.TTF");
    ui.fonts.insert_from_file(font_path).unwrap();

    let ids = &mut Ids::new(ui.widget_id_generator());
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();
    let mut events: Vec<glium::glutin::Event> = Vec::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let html = fs::read_to_string("./html/test.html")
        .unwrap()
        .replace("\r\n", "")
        .to_string();
    let mut html = Html {
        html: html,
        tag: Vec::new(),
    };

    let node = parse_node(&mut html);

    'render: loop {
        events.clear();
        event_loop.poll_events(|event| {
            events.push(event);
        });

        if !events.is_empty() {
            match events[0] {
                Event::WindowEvent {
                    event: glium::glutin::WindowEvent::CloseRequested,
                    ..
                } => break 'render,
                _ => {}
            }
        }

        set_ui(ui.set_widgets(), & ids);

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn set_ui(ref mut ui: conrod::UiCell, ids: & Ids) {
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(20.0)),
        (ids.body, widget::Canvas::new().length(300.0).flow_right(&[
            (ids.left_column, widget::Canvas::new().color(color::LIGHT_ORANGE).pad(20.0)),
            (ids.middle_column, widget::Canvas::new().color(color::ORANGE)),
            (ids.right_column, widget::Canvas::new().color(color::DARK_ORANGE).pad(20.0)),
        ])),
        (ids.footer, widget::Canvas::new().color(color::BLUE).scroll_kids_vertically()),
        ]).set(ids.master, ui);
}
