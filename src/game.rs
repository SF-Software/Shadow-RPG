/* Project @ Shadow RPG
 * game
 */


pub mod game {
    use piston::window::WindowSettings;
    use piston::event_loop::*;
    use piston::input::*;
    use glutin_window::GlutinWindow as Window;
    use opengl_graphics::{GlGraphics, OpenGL};
    use graphics::Context;

    use game_const::BLACK;
    use game_const::RED;
    use entity::Entity;
    use game::rotating_box;

    pub struct Game {
        gl: GlGraphics, // OpenGL drawing backend.
        rotating_box: rotating_box::RotatingBox,
    }

    impl Game {
        pub fn new() -> Game {
            let opengl = OpenGL::V3_2;
            Game {
                gl: GlGraphics::new(opengl),
                rotating_box: rotating_box::RotatingBox::new(),
            }
        }

        pub fn render(&mut self, args: &RenderArgs) {
            use graphics::*;
            let rb = &self.rotating_box;
            self.gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                rb.renderer(gl, &c, args)
            });
        }

        pub fn update(&mut self, args: &UpdateArgs) {
            self.rotating_box
                .process(rotating_box::Input { dt: args.dt });
        }
    }
}


pub mod rotating_box {
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
    pub struct RotatingBox {
        state: State,
        rendererState: RendererState,
    }
    impl RotatingBox {
        pub fn new() -> RotatingBox {
            RotatingBox {
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
    impl entity::Entity<State, RendererState, Input> for RotatingBox {
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
}

