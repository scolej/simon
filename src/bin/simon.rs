extern crate conrod;
extern crate simon;

use conrod::backend::glium::glium::{self, Surface};
use conrod::backend::winit::WinitWindow;
use conrod::position::Place;
use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
use std::cmp::min;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use simon::random::a_random_build;
use simon::{Build, BuildId, BuildStatus};

#[derive(PartialEq, Eq, Debug)]
enum DisplayMode {
    Tiny,        // Draw a button for each build with no label.
    Abbreviated, // Draw a button with only the build number.
    Full,        // Draw a button with build number, commit hash and build duration.
}

// Given the width and height of the window, determine what display mode to use.
fn display_mode(w: u32, h: u32) -> DisplayMode {
    let small = min(w, h);
    if small < 200 {
        DisplayMode::Tiny
    } else if small < 400 {
        DisplayMode::Abbreviated
    } else {
        DisplayMode::Full
    }
}

// Button size in pixels for a given display mode.
fn button_size(m: &DisplayMode) -> u32 {
    match m {
        DisplayMode::Full => 70,
        DisplayMode::Abbreviated => 30,
        DisplayMode::Tiny => 10,
    }
}

fn main() {
    // Get some test data.
    let mut builds = Vec::new();
    for _ in 0..30 {
        builds.push(a_random_build());
    }

    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    ui.theme.font_id = Some(
        ui.fonts
            .insert_from_file("NotoSansMono-SemiBold.ttf")
            .unwrap(),
    );

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Simon")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(1);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut branch_button_ids = HashMap::new();
    let mut build_button_ids = HashMap::new();

    let canvas_id = ui.widget_id_generator().next();

    let mut build_map = HashMap::new();
    for b in &builds {
        let e = build_map.entry(&b.id.branch);
        e.or_insert(Vec::new()).push(b);
    }

    let mut events = Vec::new();
    loop {
        events.clear();
        events_loop.poll_events(|event| {
            events.push(event);
        });
        if events.is_empty() {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        for event in events.drain(..) {
            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };
            ui.handle_event(input);
        }

        {
            let (win_width, win_height) = display.get_inner_size().unwrap();

            let current_display_mode = display_mode(win_width, win_height);

            let ui = &mut ui.set_widgets();
            let side = button_size(&current_display_mode).into();
            let wide = side * 2.0;
            let pad = 1.0;

            widget::TextBox::new("Filter branches here")
                .top_left()
                .h(40.0)
                .set(canvas_id, ui);

            for (branch, builds) in build_map.iter() {
                let branch_build_id = branch_button_ids
                    .entry(branch.to_owned())
                    .or_insert(ui.widget_id_generator().next());

                // A button for every branch.
                widget::Button::new()
                    .label(branch.to_owned())
                    .down(pad)
                    .x_place(Place::Start(None))
                    .w_h(wide, side)
                    .set(*branch_build_id, ui);

                for build in builds.iter() {
                    let widg_build_id = build_button_ids
                        .entry(build.id.clone())
                        .or_insert(ui.widget_id_generator().next());
                    let text = [build.id.number.to_string(), build.commit.to_owned()].join("\n");

                    // A button for every build.
                    let mut bb = widget::Button::new();
                    if current_display_mode != DisplayMode::Tiny {
                        bb = bb.label(&text);
                    }
                    bb.right(pad)
                        .center_justify_label()
                        .w_h(side, side)
                        .set(*widg_build_id, ui);
                }
            }
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(1.0, 1.0, 1.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}
