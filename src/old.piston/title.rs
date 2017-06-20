/* Project @ Shadow RPG
 * title
 */

use entity;
use opengl_graphics::*;
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::Context;
use piston::input::*;
use game_const::*;
use find_folder::*;

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
    select: i32,
    image: Texture,
    glyphs: GlyphCache<'static>,
}

impl Title {
    pub fn new() -> Title {
        let images = Search::ParentsThenKids(3, 3).for_folder("images").unwrap();

        let title_image = images.join("title.jpg");
        let title_image = Texture::from_path(&title_image).unwrap();
        let fonts = Search::ParentsThenKids(3, 3).for_folder("fonts").unwrap();

        let glyphs = fonts.join("NotoSansCJKtc-Light.ttf");
        let mut glyphs = GlyphCache::new(glyphs).unwrap();
        Title {
            state: State {
                color: RED,
                rotation: 1.0,
            },
            rendererState: RendererState {
                color: RED,
                rotation: 1.0,
            },
            select: 0,
            image: title_image,
            glyphs: glyphs,
        }
    }
}

impl entity::Entity<State, RendererState, Input> for Title {
    fn renderer(&mut self, gl: &mut GlGraphics, c: &Context, args: &RenderArgs) {
        use graphics::*;

        image(&self.image, c.transform, gl);

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rendererState.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let transform = c.transform.trans(x, y).rot_rad(rotation).trans(
            -25.0,
            -25.0,
        );

        rectangle(self.rendererState.color, square, transform, gl);


        let mut glyphs = &mut self.glyphs;
        text::Text::new_color(WHITE, 32).draw(
            "Start",
            glyphs,
            &c.draw_state,
            c.transform.trans(80.0, 500.0),
            gl,
        );

        text::Text::new_color(WHITE, 32).draw(
            "Load",
            glyphs,
            &c.draw_state,
            c.transform.trans(280.0, 500.0),
            gl,
        );
        text::Text::new_color(WHITE, 32).draw(
            "Settings",
            glyphs,
            &c.draw_state,
            c.transform.trans(480.0, 500.0),
            gl,
        );
        text::Text::new_color(WHITE, 32).draw(
            "Exit",
            glyphs,
            &c.draw_state,
            c.transform.trans(680.0, 500.0),
            gl,
        );
        text::Text::new_color(WHITE, 32).draw(" ", glyphs, &c.draw_state, c.transform, gl);
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
