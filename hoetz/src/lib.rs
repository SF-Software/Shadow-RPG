#![allow(dead_code)]

extern crate sdl2;
extern crate lru_time_cache;

pub mod helper;
pub mod scene;
pub mod context;
pub mod graphics;
pub mod event;
pub mod types;


use self::context::Context;
use self::scene::BoxedScene;
use self::context::ResourceContext;
use self::graphics::start as graphics_start;

use sdl2::event::Event;
use std::thread::sleep;
use self::event::UIInput;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};
use sdl2::image::{INIT_PNG, INIT_JPG};

fn update(mut scene: BoxedScene) -> BoxedScene {
    match scene.update(&UIInput {}) {
        Some(s) => s,
        None => scene,
    }
}


pub fn game_start(width: u32, height: u32, title: String, mut current_scene: BoxedScene, fps: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(&title, width, height)
        .position_centered()
        .build()
        .unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let ns_per_frame: Duration = Duration::new(0, 1_000_000_000 / fps);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;


    let _ = graphics_start(
        window.into_canvas().accelerated().build().unwrap(),
        |graphics| {
            let c = Context::new(graphics);
            let rc = ResourceContext::new(graphics);
            while running {
                let start = Instant::now();
                let next_render_step = start + ns_per_frame;

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            running = false;
                        }
                        _ => {}
                    }
                }
                current_scene.resource_load(&rc);
                current_scene = update(current_scene);
                graphics.render(|| { current_scene.render_view(&c); });
                let now = Instant::now();
                if next_render_step >= now {
                    sleep(next_render_step - now);
                }
            }
        },
    );


}
