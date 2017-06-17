use super::{ResourceManager, ResourceLoader};

use sdl2::ttf::{FontStyle, Sdl2TtfContext, Font};
use sdl2::render::{TextureCreator, Texture};
use sdl2::pixels::Color;
use std::cell::RefCell;

pub type FontManager<'l> = ResourceManager<'l,
                                           FontDetails,
                                           RefCell<Font<'l, 'static>>,
                                           Sdl2TtfContext>;
pub type GlyphManager<'l, T> = ResourceManager<'l, GlyphDetails, Texture<'l>, GlyphCreator<'l, T>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path.clone(),
            size: details.size,
        }
    }
}

impl<'l> ResourceLoader<'l, RefCell<Font<'l, 'static>>> for Sdl2TtfContext {
    type Args = FontDetails;
    fn load(&'l self, details: &FontDetails) -> Result<RefCell<Font<'l, 'static>>, String> {
        Ok(RefCell::new(self.load_font(&details.path, details.size).unwrap()))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GlyphDetails {
    pub font: FontDetails,
    pub character: char,
    pub style: FontStyle,
}

impl<'a> From<&'a GlyphDetails> for GlyphDetails {
    fn from(details: &'a GlyphDetails) -> GlyphDetails {
        GlyphDetails {
            font: details.font.clone(),
            character: details.character,
            style: details.style.clone(),
        }
    }
}

pub struct GlyphCreator<'l, T: 'l> {
    font_manager: RefCell<FontManager<'l>>,
    texture_creator: &'l TextureCreator<T>,
}
impl<'l, T: 'l> GlyphCreator<'l, T> {
    pub fn new(context: &'l mut Sdl2TtfContext, tc: &'l TextureCreator<T>) -> Self {
        GlyphCreator {
            font_manager: RefCell::new(FontManager::new(context, 5)),
            texture_creator: tc,
        }
    }
}

impl<'l, T> ResourceLoader<'l, Texture<'l>> for GlyphCreator<'l, T> {
    type Args = GlyphDetails;
    fn load(&'l self, details: &GlyphDetails) -> Result<Texture<'l>, String> {
        let mut ftm = self.font_manager.borrow_mut();
        let f = ftm.get(&details.font);
        let mut f = f.borrow_mut();
        f.set_style(details.style);
        let s = f.render_char(details.character)
            .blended(Color::RGBA(255, 0, 0, 255))
            .unwrap();
        let t = self.texture_creator.create_texture_from_surface(s).unwrap();
        Ok(t)
    }
}

