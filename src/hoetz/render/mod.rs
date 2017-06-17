mod resource_manager;

use std::cmp::{Ord, Ordering};

use std::path::Path;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{TextureCreator, Texture, TextureQuery};
use sdl2::ttf::Font;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::ttf;
use sdl2::ttf::{FontStyle, Sdl2TtfContext};

use self::resource_manager::font::{GlyphDetails, FontDetails,GlyphManager,GlyphCreator};
use self::resource_manager::texture::{TextureManager};

use lru_time_cache::LruCache;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub struct Renderer<'l> {
    canvas: Canvas<Window>,
    texture_manager: Option<TextureManager<'l,WindowContext>>,
    glyph_manager: Option<GlyphManager<'l,WindowContext>>,
    texture_creator: TextureCreator<WindowContext>,
    glyph_creator: Option<GlyphCreator<'l, WindowContext>>,
}

impl<'l> Renderer<'l> {
    // unusable for now
    pub fn new(canvas: Canvas<Window>) -> Renderer<'l> {
        let texture_creator= canvas.texture_creator();
        //let glyph_creator = GlyphCreator::new(ttf::init().unwrap(), &mut texture_creator);

        let mut s = Renderer {
            canvas: canvas,
            texture_manager: None,//TextureManager::new(&mut texture_creator,300),
            glyph_manager: None,//GlyphManager::new(&mut glyph_creator, 3000),
            texture_creator: texture_creator,
            glyph_creator: None,//glyph_creator,
        };
        s.glyph_creator =Some(GlyphCreator::new(ttf::init().unwrap(), &mut s.texture_creator));
        s
    }
    pub fn render<F>(&mut self, mut f: F)
        where F: FnMut(&mut Renderer)
    {
        self.canvas.clear();
        f(self);
        self.canvas.present();
    }
}

/*

impl<'l,T> Renderer<'l,T> {
    pub fn text(&'l mut  self,
                s: String,
                font: String,
                size: u16,
                x: i32,
                y: i32,
                color: Color,
                style: FontStyle) {

        let mut r = rect!(x, y, 0, 0);
        
        for c in s.chars() {

            if c == '\n' {
                let b = r.bottom();
                r.set_y(b);
                r.set_x(x);
            } else {
                let g = GlyphDetails{font:FontDetails{path:font,size:size},character:c,style:style};
               
                if !self.glyph_cache.contains_key(&g) {
                    if !self.font_cache.contains_key(&font) {
                        let f = self.ttf_context
                            .load_font(Path::new(&format!("fonts/{}", &font)), 128)
                            .unwrap();
                        self.font_cache.insert(font.clone(), f);
                    }
                    let f = self.font_cache.get(&font).unwrap();
                    let s = f.render_char(c)
                        .blended(Color::RGBA(255, 0, 0, 255))
                        .unwrap();
                    let t = self.texture_creator.create_texture_from_surface(s).unwrap();
                    self.glyph_cache.insert(g.clone(), t);
                }
                let texture = self.glyph_cache.get(&g).unwrap();
                let TextureQuery{format:_,access:_,width:w, height:h} = texture.query();
                r.set_width(w);
                r.set_height(h);
                self.canvas.copy(texture, Option::None, r);
                let rr=r.right();
                r.set_x(rr);
            }
        }
    }
}*/
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