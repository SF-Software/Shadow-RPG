pub mod text;
mod resource_manager;

use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{TextureCreator, TextureQuery};
use sdl2::rect::Rect;
use sdl2::ttf;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
use self::resource_manager::{GlyphManager, GlyphCreator, TextureManager};

pub struct Renderer<'l> {
    canvas: Canvas<Window>,
    texture_manager: TextureManager<'l, WindowContext>,
    glyph_manager: GlyphManager<'l,'l, WindowContext>,
}

impl<'l> Renderer<'l> {
    fn new(canvas: Canvas<Window>,
           texture_creator: TextureCreator<WindowContext>,
           glyph_creator: GlyphCreator<'l, WindowContext>)
           -> Renderer<'l> {
        Renderer {
            canvas: canvas,
            texture_manager: TextureManager::new(texture_creator),
            glyph_manager: GlyphManager::new(glyph_creator),
        }

    }
  
    pub fn render<F>(&'l mut self, mut f: F)
        where F: FnOnce(&mut Renderer)
    {
        self.canvas.clear();
        self.image_from_file(String::from("title.jpg"),0,0);
        f(self);
        self.canvas.present();
    }
}
  pub fn start<F: FnOnce(&mut Renderer)>(canvas: Canvas<Window>, callback: F) {
        let ttf_context = ttf::init().unwrap();
        let texture_creator1 = canvas.texture_creator();
        
        let texture_creator2 = canvas.texture_creator();
        let glyph_creator = GlyphCreator::new(ttf_context, texture_creator1);
        let mut r = Renderer::new(canvas, texture_creator2, glyph_creator);
        
        callback(&mut r);
    }


impl<'l> Renderer<'l> {
    pub fn image_from_file(&'l mut self, file: String, x: i32, y: i32) {
        let t = self.texture_manager.get(file);
        let TextureQuery{access:_,format:_,width:w,height:h} = t.query();
        self.canvas.copy(&t,None,rect!(x,y,w,h));
    }
}


