
use lru_time_cache::LruCache;
use std::cmp::Ord;

use std::cell::RefCell;
use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, Texture};
use std::rc::Rc;

pub fn idc<T>(x: &T)->T
    where T: Clone
{
    x.clone()
}

pub struct TextureManager<'r, T> where T:'r {
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
        let mut c =self.cache.borrow_mut();
        if !c.contains_key(&key) {
            let resource = Rc::new(self.creator.load_texture(format!("images/{}",&key)).unwrap());
            c.insert(key.clone(), resource.clone());
        }
        c.get(&key).unwrap().clone()
    }
}

use sdl2::ttf::{FontStyle, Sdl2TtfContext, Font};
use sdl2::pixels::Color;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}



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
    pub fn get(&self, key: FontDetails) -> Rc<RefCell<Font<'r,'static>>> {
        let path = key.path.clone();
        let size = key.size;
        self.cache
            .borrow_mut()
            .get(&key)
            .map_or_else(|| {
                             let resource = Rc::new(RefCell::new(self.creator.load_font(path,size).unwrap()));
                             self.cache.borrow_mut().insert(key, resource.clone());
                             resource
                         },
                         idc)
    }
   
}

pub struct GlyphCreator<'r, T:'r> {
    font_manager: FontManager<'r>,
    texture_creator: &'r TextureCreator<T>,
}
impl<'r, T> GlyphCreator<'r, T>
{
    pub fn new(font_manager: FontManager<'r>, texture_creator: &'r TextureCreator<T>) -> Self {
        Self {
            font_manager: font_manager,
            texture_creator: texture_creator,
        }
    }
    pub fn load_glyph(&self, details: &GlyphDetails) -> Result<Texture<'r>, String> {

        let f = self.font_manager.get(details.font.clone());
        f.borrow_mut().set_style(details.style);
        let s = f.borrow().render_char(details.character)
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let t = self.texture_creator.create_texture_from_surface(s).unwrap();
        Ok(t)
    }
}

pub struct GlyphManager<'r, T> where T:'r{
    creator: GlyphCreator<'r, T>,
    cache: RefCell<LruCache<GlyphDetails, Rc<Texture<'r>>>>,
}
impl<'r, T> GlyphManager<'r, T>
{
    pub fn new(creator: GlyphCreator<'r, T>) -> Self {
        GlyphManager {
            creator: creator,
            cache: RefCell::new(LruCache::with_capacity(2000)),
        }
    }
    pub fn get(&self, key: GlyphDetails) -> Rc<Texture> {
        self.cache
            .borrow_mut()
            .get(&key)
            .map_or_else(|| {
                             let resource = Rc::new(self.creator.load_glyph(&key).unwrap());
                             self.cache.borrow_mut().insert(key, resource.clone());
                             resource
                         },
                         idc)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GlyphDetails {
    pub font: FontDetails,
    pub character: char,
    pub style: FontStyle,
}
#[cfg(test)]
mod old {
    pub struct ResourceManager<'l, K, R, L>
        where K: Ord + Clone,
              L: 'l + ResourceLoader<'l, R>
    {
        loader: &'l L,
        cache: LruCache<K, Rc<R>>,
    }

    impl<'l, K, R, L> ResourceManager<'l, K, R, L>
        where K: Ord + Clone,
              L: ResourceLoader<'l, R>
    {
        pub fn new(loader: &'l L, n: usize) -> Self {
            ResourceManager {
                cache: LruCache::with_capacity(n),
                loader: loader,
            }
        }
        pub fn get<D>(&mut self, details: &D) -> Rc<R>
            where L: ResourceLoader<'l, R, Args = D>,
                  D: Ord + Clone,
                  K: Borrow<D> + for<'a> From<&'a D>
        {
            if !self.cache.contains_key(details) {
                let res = Rc::new(self.loader.load(details).unwrap());
                self.cache.insert(details.into(), res);
            }
            self.cache.get(details).unwrap().clone()
        }
    }




    pub trait ResourceLoader<'l, R> {
        type Args: ?Sized;
        fn load(&'l self, data: &Self::Args) -> Result<R, String>;
    }


}

