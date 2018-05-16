extern crate conrod;
extern crate rand;

use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

mod model;
mod provider;
mod random;

use model::{Build, BuildId, BuildStatus};
use random::a_random_build;

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
        .with_title("Hello Conrod!")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(1);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut ids = HashMap::new();
    for b in &builds {
        ids.insert(&b.id, b);
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

            // widget::Button::new()
            //     .label("1skyrz-tb01/develop")
            //     .top_left()
            //     .right_justify_label()
            //     .w_h(wide, side)
            //     .set(ids.text, ui);

            // widget::Button::new()
            //     .label("47\n3243fa6\n00:15")
            //     .center_justify_label()
            //     .w_h(side, side)
            //     .right(pad)
            //     .set(ids.but1, ui);

            // widget::Button::new()
            //     .label("48\n3243fa6\n00:15")
            //     .center_justify_label()
            //     .w_h(side, side)
            //     .right(pad)
            //     .set(ids.but2, ui);

            // widget::Button::new()
            //     .label("1skyrz-tb01/develop")
            //     .down(pad)
            //     .x_place(conrod::position::Place::Start(None))
            //     .right_justify_label()
            //     .w_h(wide, side)
            //     .set(ids.text2, ui);
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
