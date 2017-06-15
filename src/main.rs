/* Project @ Shadow RPG
 * @ SF Software
 */
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod entity;
mod title;
mod game_const;
mod game;
mod hoetz;

use game::Game;
use title::Title;

fn main() {
    /* piston = "0.32.0"
piston2d-graphics = "0.21.1"
pistoncore-glutin_window = "0.37.0"
piston2d-opengl_graphics = "0.43.1"*/
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Shadow RPG", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game::new();

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}

