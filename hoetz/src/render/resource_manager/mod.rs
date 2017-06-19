
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

pub struct TextureManager<'a, T> {
    creator: TextureCreator<T>,
    cache: RefCell<LruCache<String, Rc<Texture<'a>>>>,
}

impl<'a, T> TextureManager<'a, T> {
    pub fn new(creator: TextureCreator<T>) -> Self {
        Self {
            creator: creator,
            cache: RefCell::new(LruCache::with_capacity(500)),
        }
    }
    pub fn get(&'a self, key: String) -> Rc<Texture> {
        self.cache
            .borrow_mut()
            .get(&key)
            .map_or_else(|| {
                             let resource = Rc::new(self.creator.load_texture(format!("images/{}",&key)).unwrap());
                             self.cache.borrow_mut().insert(key, resource.clone());
                             resource
                         },
                         idc)
    }
}

use sdl2::ttf::{FontStyle, Sdl2TtfContext, Font};
use sdl2::pixels::Color;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}



pub struct FontManager<'ttf> {
    creator: Sdl2TtfContext,
    cache: RefCell<LruCache<FontDetails, Rc<RefCell<Font<'ttf, 'static>>>>>,
}
impl<'ttf> FontManager<'ttf> {
    pub fn new(creator: Sdl2TtfContext) -> Self {
        Self {
            creator: creator,
            cache: RefCell::new(LruCache::with_capacity(10)),
        }
    }
    pub fn get(&'ttf self, key: FontDetails) -> Rc<RefCell<Font<'ttf,'static>>> {
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

pub struct GlyphCreator<'ttf, T> {
    font_manager: FontManager<'ttf>,
    texture_creator: TextureCreator<T>,
}
impl<'ttf, 'a, T> GlyphCreator<'ttf, T>
    where 'a: 'ttf
{
    pub fn new(context: Sdl2TtfContext, texture_creator: TextureCreator<T>) -> Self {
        Self {
            font_manager: FontManager::new(context),
            texture_creator: texture_creator,
        }
    }
    pub fn load_glyph(&'a self, details: &GlyphDetails) -> Result<Texture<'a>, String> {

        let f = self.font_manager.get(details.font.clone());
        f.borrow_mut().set_style(details.style);
        let s = f.borrow().render_char(details.character)
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let t = self.texture_creator.create_texture_from_surface(s).unwrap();
        Ok(t)
    }
}

pub struct GlyphManager<'ttf, 'a, T> {
    creator: GlyphCreator<'ttf, T>,
    cache: RefCell<LruCache<GlyphDetails, Rc<Texture<'a>>>>,
}
impl<'ttf, 'a, T> GlyphManager<'ttf, 'a, T>
    where 'a: 'ttf
{
    pub fn new(creator: GlyphCreator<'ttf, T>) -> Self {
        GlyphManager {
            creator: creator,
            cache: RefCell::new(LruCache::with_capacity(2000)),
        }
    }
    pub fn get(&'a self, key: GlyphDetails) -> Rc<Texture> {
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

