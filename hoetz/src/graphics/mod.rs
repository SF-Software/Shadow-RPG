
mod resource_manager;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf;
use std::cell::RefCell;

use self::resource_manager::{GlyphManager, FontManager, GlyphCreator, TextureManager};

pub struct Graphics<'t> {
    pub canvas: RefCell<Canvas<Window>>,
    pub texture_manager: RefCell<TextureManager<'t, WindowContext>>,
    pub glyph_manager: RefCell<GlyphManager<'t, WindowContext>>,
}

impl<'r, 't> Graphics<'t>
where
    'r: 't,
{
    fn new(
        canvas: Canvas<Window>,
        texture_creator: &'r TextureCreator<WindowContext>,
        glyph_creator: GlyphCreator<'t, WindowContext>,
    ) -> Graphics<'t> {
        Self {
            canvas: RefCell::new(canvas),
            texture_manager: RefCell::new(TextureManager::new(texture_creator)),
            glyph_manager: RefCell::new(GlyphManager::new(glyph_creator)),
        }

    }

    pub fn render<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        self.canvas.borrow_mut().clear();
        f();
        self.canvas.borrow_mut().present();
    }
}
pub fn start<F: FnOnce(&Graphics)>(canvas: Canvas<Window>, callback: F) {
    let ttf_context = ttf::init().unwrap();
    let texture_creator = canvas.texture_creator();
    let font_manager = FontManager::new(&ttf_context);
    let glyph_creator = GlyphCreator::new(font_manager, &texture_creator);

    let r = Graphics::new(canvas, &texture_creator, glyph_creator);
    callback(&r);
}

/*
impl<'t> Graphics<'t> {
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
*/
