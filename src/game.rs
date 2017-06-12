/* Project @ Shadow RPG
 * game
 */

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::*;
use graphics::Context;
use find_folder::*;

use game_const::BLACK;
use game_const::RED;
use entity::Entity;
use title;

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    title: title::Title,
}

impl Game {
    pub fn new() -> Game {
        let opengl = OpenGL::V3_2;
        Game {
            gl: GlGraphics::new(opengl),
            title: title::Title::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let rb = &self.title;
        self.gl.draw(args.viewport(), |c, gl| {
            rb.renderer(gl, &c, args)
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.title
            .process(title::Input { dt: args.dt });
    }
}
