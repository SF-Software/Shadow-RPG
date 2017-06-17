pub mod text;
mod resource_manager;

use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::render::TextureCreator;

use sdl2::ttf;


use self::resource_manager::font::{GlyphManager, GlyphCreator};
use self::resource_manager::texture::TextureManager;



pub struct Renderer<'l> {
    canvas: Canvas<Window>,
    texture_manager: TextureManager<'l, WindowContext>,
    glyph_manager: GlyphManager<'l, WindowContext>,
}

impl<'l> Renderer<'l> {
    fn new(canvas: Canvas<Window>,
           texture_creator: &'l mut TextureCreator<WindowContext>,
           glyph_creator: &'l mut GlyphCreator<'l, WindowContext>)
           -> Renderer<'l> {
        Renderer {
            canvas: canvas,
            texture_manager: TextureManager::new(texture_creator, 300),
            glyph_manager: GlyphManager::new(glyph_creator, 3000),
        }

    }
  
    pub fn render<F>(&mut self, mut f: F)
        where F: FnMut(&mut Renderer)
    {
        self.canvas.clear();
        f(self);
        self.canvas.present();
    }
}
  pub fn start<F: FnOnce(&mut Renderer)>(canvas: Canvas<Window>, callback: F) {
        let mut ttf_context = ttf::init().unwrap();
        let texture_creator1 = canvas.texture_creator();
        {
        let mut texture_creator2 = canvas.texture_creator();
        let mut glyph_creator = GlyphCreator::new(&mut ttf_context, &texture_creator1);
        let mut r = Renderer::new(canvas, &mut texture_creator2, &mut glyph_creator);
        }
        //callback(&mut r);
    }

/*
impl Renderer {
    pub fn image_from_file(&mut self, file: String, x: i32, y: i32) {
        if !self.glyph_cache.contains_key(&g) {
         //  let t=texture_creator.load_texture(Path::new(format!("images/{}", file))).unwrap();
                self.font_cache.insert(font, f);
            
        }
    }
}

*/
