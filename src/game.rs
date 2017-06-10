/* Project @ Shadow RPG 
 * game
 */
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use game_const::BLACK;
use game_const::RED;

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64
}

impl Game {
    pub fn new() -> Game {
        let opengl = OpenGL::V3_2;
        Game {
            gl: GlGraphics::new(opengl),
            rotation: 0.0
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box.
            rectangle(RED, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Status: title / map / battle / camp ...
        // Do something
        self.rotation += 2.0 * args.dt;

    }
}
