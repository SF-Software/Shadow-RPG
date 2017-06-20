#![allow(dead_code)]

extern crate sdl2;
extern crate lru_time_cache;

pub mod scene;
pub mod render;


use self::render::{start as render_start};
use self::scene::BoxedScene;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{INIT_PNG, INIT_JPG};
use std::time::{Duration, Instant};
use std::thread::sleep;
use self::scene::UIInput;

fn update(mut scene: BoxedScene) -> BoxedScene {
    match scene.update(UIInput {}) {
        Some(s) => s,
        None => scene,
    }
}


pub fn game_start(mut current_scene: BoxedScene, fps: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SDL2", 640, 480)
        .position_centered()
        .build()
        .unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let ns_per_frame: Duration = Duration::new(0, 1_000_000_000 / fps);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;


    let _ =
        render_start(window.into_canvas().accelerated().build().unwrap(),
                     |renderer| while running {
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

                         current_scene = update(current_scene);
                         renderer.render(|r| { current_scene.render_view(r); });
                         let now = Instant::now();
                         if next_render_step >= now {
                             sleep(next_render_step - now);
                         }
                     });


}

