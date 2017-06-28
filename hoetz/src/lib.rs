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
use self::event::EventSystem;
use self::graphics::start as graphics_start;

use std::thread::sleep;
use self::event::UIInput;
use std::time::{Duration, Instant};
use sdl2::image::{INIT_PNG, INIT_JPG};

fn update(mut scene: BoxedScene, input: UIInput) -> BoxedScene {
    match scene.update(&input) {
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

    let event_pump = sdl_context.event_pump().unwrap();
    graphics_start(
        window.into_canvas().accelerated().build().unwrap(),
        |graphics| {
            let c = Context::new(graphics);
            let rc = ResourceContext::new(graphics);
            let mut event_system = EventSystem::new(event_pump);
            let mut running = true;
            
            let mut frame:u32 = 0;
            while running {
                let start = Instant::now();
                let next_render_step = start + ns_per_frame;

                
                current_scene.resource_load(&rc);
                current_scene = update(current_scene, event_system.process(&mut running));
                graphics.render(|| { current_scene.render_view(&c, frame); });
                let now = Instant::now();
                if next_render_step >= now {
                    sleep(next_render_step - now);
                }
                frame += 1;
            }
        },
    );


}
