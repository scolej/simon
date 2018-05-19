#[macro_use]
extern crate conrod;
extern crate simon;

use conrod::backend::glium::glium::{self, Surface};
use conrod::position::Place;
use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use simon::random::a_random_build;
use simon::{Build, BuildId, BuildStatus};

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

    // widget_ids! {
    //     struct Ids {
    //         branch_buttons[],
    //         build_buttons[],
    //     }
    // }
    // let mut ids = Ids::new(ui.widget_id_generator());

    let mut branch_button_ids = HashMap::new();
    let mut build_button_ids = HashMap::new();
    let canvas_id = ui.widget_id_generator().next();

    let mut build_map = HashMap::new();
    for b in &builds {
        let e = build_map.entry(&b.id.branch);
        e.or_insert(Vec::new()).push(b);
    }

    // FIXME This awful dirty loop.
    loop {
        events_loop.poll_events(|event| {
            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => return,
                Some(input) => input,
            };
            ui.handle_event(input);
        });

        {
            let ui = &mut ui.set_widgets();
            let side = 70.0;
            let wide = 280.0;
            let pad = 1.0;

            widget::TextBox::new("Filter branches here")
                .top_left()
                .h(40.0)
                .set(canvas_id, ui);

            for (branch, builds) in build_map.iter() {
                let branch_build_id = branch_button_ids
                    .entry(branch.to_owned())
                    .or_insert(ui.widget_id_generator().next());
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
                    let text = [
                        // build.id.branch.to_owned(),
                        build.id.number.to_string(),
                        build.commit.to_owned(),
                    ].join("\n");
                    widget::Button::new()
                        .label(&text)
                        .right(pad)
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

        sleep(Duration::from_secs(1))
    }
}
