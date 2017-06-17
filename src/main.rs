/* Project @ Shadow RPG
 * @ SF Software
 */

#![allow(dead_code)]
extern crate sdl2;
extern crate lru_time_cache;

mod hoetz;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2", 640, 480)
        .position_centered()
        .build()
        .unwrap();
    let _image_context = sdl2::image::init(INIT_PNG | INIT_JPG).unwrap();
    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

    let mut timer = sdl_context.timer().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();


    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture(Path::new("images/title.jpg"))
        .unwrap();
    let center = Point::new(320, 240);
    let sdl2::render::TextureQuery {
        format: _,
        access: _,
        width: width,
        height: height,
    } = texture.query();
    let mut source_rect = Rect::new(0, 0, width, height);
    let mut dest_rect = Rect::new(0, 0, width, height);
    dest_rect.center_on(center);

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                }
                _ => {}
            }
        }

        let ticks = timer.ticks();

        canvas.clear();
        canvas
            .copy_ex(&texture,
                     Some(source_rect),
                     Some(dest_rect),
                     0.0,
                     None,
                     false,
                     false)
            .unwrap();
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }
}

