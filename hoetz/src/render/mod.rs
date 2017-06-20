pub mod text;
mod resource_manager;
use sdl2;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator, TextureQuery};
use sdl2::ttf;

pub type Color = sdl2::pixels::Color;
pub type Rect = sdl2::rect::Rect;

use self::resource_manager::{GlyphManager, FontManager, GlyphCreator, TextureManager};

pub struct Renderer<'t> {
    canvas: Canvas<Window>,
    texture_manager: TextureManager<'t, WindowContext>,
    glyph_manager: GlyphManager<'t, WindowContext>,
}

impl<'r, 't> Renderer<'t>
where
    'r: 't,
{
    fn new(
        canvas: Canvas<Window>,
        texture_creator: &'r TextureCreator<WindowContext>,
        glyph_creator: GlyphCreator<'t, WindowContext>,
    ) -> Renderer<'t> {
        Renderer {
            canvas: canvas,
            texture_manager: TextureManager::new(texture_creator),
            glyph_manager: GlyphManager::new(glyph_creator),
        }

    }

    pub fn render<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Renderer),
    {
        self.canvas.clear();

        f(self);
        self.canvas.present();
    }
}
pub fn start<F: FnOnce(&mut Renderer)>(canvas: Canvas<Window>, callback: F) {
    let ttf_context = ttf::init().unwrap();
    let texture_creator = canvas.texture_creator();
    let font_manager = FontManager::new(&ttf_context);
    let glyph_creator = GlyphCreator::new(font_manager, &texture_creator);

    let mut r = Renderer::new(canvas, &texture_creator, glyph_creator);
    callback(&mut r);
}


impl<'t> Renderer<'t> {
    pub fn image_from_file(&mut self, file: String, x: i32, y: i32) {
        let t = self.texture_manager.get(file);
        let TextureQuery {
            access: _,
            format: _,
            width: w,
            height: h,
        } = t.query();
        let _ = self.canvas.copy(
            &t,
            None,
            Rect::new(x as i32, y as i32, w as u32, h as u32),
        );
    }
}
