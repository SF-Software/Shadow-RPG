/* Project @ Shadow RPG
 * title
 */

use entity;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Context;
use piston::input::*;
use game_const::RED;

#[derive(Clone)]
struct State {
	color: [f32; 4],
	rotation: f64,
}

impl entity::State for State {}
struct RendererState {
	color: [f32; 4],
	rotation: f64,
}

impl entity::RendererState for RendererState {}

pub struct Input {
	pub dt: f64,
}

impl entity::Input for Input {}

pub struct Title {
	state: State,
	rendererState: RendererState,
}

impl Title {
	pub fn new() -> Title {
		Title {
			state: State {
				color: RED,
                rotation: 1.0,
            },
            rendererState: RendererState {
                color: RED,
                rotation: 1.0,
            },
        }
    }
}

impl entity::Entity<State, RendererState, Input> for Title {
	fn renderer(&self, gl: &mut GlGraphics, c: &Context, args: &RenderArgs) {
		use graphics::*;
		let square = rectangle::square(0.0, 0.0, 50.0);
		let rotation = self.rendererState.rotation;
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
		let transform = c.transform
			.trans(x, y)
			.rot_rad(rotation)
			.trans(-25.0, -25.0);

		rectangle(self.rendererState.color, square, transform, gl);
	}
	fn update(s: &State, i: &Input) -> (State, RendererState, entity::CurrentState) {
		let new_state = State {
			rotation: s.rotation + 0.1,
			..*s
		};
		let renderer_state = RendererState {
            rotation: new_state.rotation,
            color: new_state.color,
        };
        (new_state, renderer_state, entity::CurrentState::Running)
    }

    fn process(&mut self, i: Input) {
        let (s, r, c) = Self::update(&self.state, &i);
        self.state = s;
        self.rendererState = r;
    }
}
