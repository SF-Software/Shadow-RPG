mod scene;
mod renderer;

use self::scene::*;
use self::renderer::*;

use piston::input::*;
use opengl_graphics::*;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use piston::event_loop::*;

fn update(scene: Box<Scene>, args: &UpdateArgs) -> Box<Scene> {
    let mut scene = scene;
    match scene.update(UIInput {}) {
        Some(s) => s,
        None => scene,
    }
}
fn render(args: &RenderArgs) {}


pub fn game_start(scene: Box<Scene>) {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Shadow RPG", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut events = Events::new(EventSettings::new());
    let mut scene = scene;
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            render(&r);
        }
        if let Some(u) = e.update_args() {
            scene = update(scene, &u);
        }
    }
}

