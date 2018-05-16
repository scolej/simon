extern crate conrod;
extern crate rand;

#[macro_use]
extern crate rand_derive;

use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Labelable, Positionable, Sizeable, Widget};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

mod model;
mod provider;

use model::{Build, BuildStatus};

fn random_status() -> BuildStatus {
    static STATUSES: [BuildStatus; 3] = [
        BuildStatus::InProgress,
        BuildStatus::Failed,
        BuildStatus::Passed,
    ];
    STATUSES[rand(STATUSES.len())]
}

fn rand(max: usize) -> usize {
    rand::thread_rng().gen::<usize>() % max
}

fn random_branch() -> String {
    static BRANCHES: [&str; 6] = [
        "master",
        "develop",
        "feature/the-best-thing",
        "feature/the-biggest-thing",
        "feature/the-fastest-thing",
        "feature/the-most-stylish-thing",
    ];
    BRANCHES[rand(BRANCHES.len())].to_string()
}

fn random_commit() -> String {
    let chars = [
        'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
    ];
    let mut s = String::new();
    for n in 0..10 {
        let i: usize = rand::thread_rng().gen::<usize>() % chars.len();
        s.push(chars[i]);
    }
    s
}

fn a_random_build() -> Build {
    Build {
        id: BuildId {
            branch: random_branch(),
            number: rand::thread_rng().gen(),
        },
        commit: random_commit(),
        status: random_status(),
        elapsed_time: Duration::from_secs(rand::thread_rng().gen()),
    }
}

fn main() {
    for _ in 0..10 {
        println!("{:?}", a_random_build());
    }
}

// DI-Lan! No judgment please, there is not even a proper event loop yet :)
// Maybe it doesn't even compile!
fn main2() {
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

    // let mut ids = HashMap::new();

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
