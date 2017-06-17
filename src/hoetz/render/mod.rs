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

use lru_time_cache::LruCache;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub struct Renderer<'r> {
    canvas: Canvas<Window>,
    ttf_context: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
    glyph_cache: LruCache<GlyphInfo, Texture<'r>>,
    font_cache: LruCache<String, Font<'r, 'static>>,
    image_cache: LruCache<String, Font<'r, 'static>>,
}

impl<'r> Renderer<'r> {
    pub fn new(canvas: Canvas<Window>) -> Renderer<'r> {

        Renderer {
            ttf_context: ttf::init().unwrap(),
            texture_creator: canvas.texture_creator(),
            canvas: canvas,
            glyph_cache: LruCache::with_capacity(3000),
            font_cache: LruCache::with_capacity(5),
            image_cache: LruCache::with_capacity(300),
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

#[derive(Clone)]
struct GlyphInfo {
    character: char,
    font: String,
    size: u32,
    style: FontStyle,
}
impl PartialEq for GlyphInfo {
    fn eq(&self, other: &GlyphInfo) -> bool {
        self.character == other.character && self.font == other.font && self.size==other.size && self.style==other.style
    }
}
impl Eq for GlyphInfo {}
impl PartialOrd for GlyphInfo{
    fn partial_cmp(&self, other :&GlyphInfo) -> Option<Ordering>{
        Some(self.cmp(other))
    }
}
impl Ord for GlyphInfo {
    fn cmp(&self, other: &GlyphInfo) -> Ordering {
        match self.font.cmp(&other.font) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                match self.size.cmp(&other.size) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => {
                        match self.style.cmp(&other.style) {
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                            Ordering::Equal => self.character.cmp(&other.character),
                        }
                    }
                }
            }
        }
    }
}




impl<'r> Renderer<'r> {
    pub fn text(&'r mut  self,
                s: String,
                font: String,
                size: u32,
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
                let g = GlyphInfo {
                    font: font.clone(),
                    size: size,
                    character: ' ',
                    style: style,
                };
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