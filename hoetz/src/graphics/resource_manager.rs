
use lru_time_cache::LruCache;

use std::cell::RefCell;
use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, Texture};
use std::rc::Rc;
use types::font::{FontDetails, GlyphDetails};

pub fn idc<T>(x: &T) -> T
where
    T: Clone,
{
    x.clone()
}

pub struct TextureManager<'r, T>
where
    T: 'r,
{
    creator: &'r TextureCreator<T>,
    cache: RefCell<LruCache<String, Rc<Texture<'r>>>>,
}

impl<'r, T> TextureManager<'r, T> {
    pub fn new(creator: &'r TextureCreator<T>) -> Self {
        Self {
            creator: creator,
            cache: RefCell::new(LruCache::with_capacity(500)),
        }
    }
    pub fn get(&self, key: String) -> Rc<Texture> {
        let mut c = self.cache.borrow_mut();
        if !c.contains_key(&key) {
            let resource = Rc::new(
                self.creator
                    .load_texture(format!("images/{}", &key))
                    .unwrap(),
            );
            c.insert(key.clone(), resource.clone());
        }
        c.get(&key).unwrap().clone()
    }
}

use sdl2::ttf::{Sdl2TtfContext, Font};
use sdl2::pixels::Color;


pub struct FontManager<'r> {
    creator: &'r Sdl2TtfContext,
    cache: RefCell<LruCache<FontDetails, Rc<RefCell<Font<'r, 'static>>>>>,
}
impl<'r> FontManager<'r> {
    pub fn new(creator: &'r Sdl2TtfContext) -> Self {
        Self {
            creator: creator,
            cache: RefCell::new(LruCache::with_capacity(10)),
        }
    }
    pub fn get(&self, key: FontDetails) -> Rc<RefCell<Font<'r, 'static>>> {
        let path = key.path.clone();
        let size = key.size;
        let mut c = self.cache.borrow_mut();
        if !c.contains_key(&key) {
            let resource = Rc::new(RefCell::new(
                self.creator
                    .load_font(format!("fonts/{}", &path), size)
                    .unwrap(),
            ));
            c.insert(key.clone(), resource.clone());
        }
        c.get(&key).unwrap().clone()

    }
}

pub struct GlyphCreator<'r, T: 'r> {
    font_manager: FontManager<'r>,
    texture_creator: &'r TextureCreator<T>,
}
impl<'r, T> GlyphCreator<'r, T> {
    pub fn new(font_manager: FontManager<'r>, texture_creator: &'r TextureCreator<T>) -> Self {
        Self {
            font_manager: font_manager,
            texture_creator: texture_creator,
        }
    }
    pub fn load_glyph(&self, details: &GlyphDetails) -> Result<Texture<'r>, String> {

        let f = self.font_manager.get(details.font.clone());
        let mut f = f.borrow_mut();
        f.set_style(details.style);
        let s = f.render_char(details.character)
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let t = self.texture_creator.create_texture_from_surface(s).unwrap();
        Ok(t)
    }
}

pub struct GlyphManager<'r, T>
where
    T: 'r,
{
    creator: GlyphCreator<'r, T>,
    cache: RefCell<LruCache<GlyphDetails, Rc<RefCell<Texture<'r>>>>>,
}
impl<'r, T> GlyphManager<'r, T> {
    pub fn new(creator: GlyphCreator<'r, T>) -> Self {
        GlyphManager {
            creator: creator,
            cache: RefCell::new(LruCache::with_capacity(2000)),
        }
    }
    pub fn get(&self, key: GlyphDetails) -> Rc<RefCell<Texture<'r>>> {
        let mut c = self.cache.borrow_mut();
        if !c.contains_key(&key) {
            let resource = Rc::new(RefCell::new(self.creator.load_glyph(&key).unwrap()));
            c.insert(key.clone(), resource.clone());
        }
        c.get(&key).unwrap().clone()
    }
}
