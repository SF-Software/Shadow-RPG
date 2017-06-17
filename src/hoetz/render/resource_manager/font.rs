use super::{ResourceManager, ResourceLoader};

use sdl2::ttf::{FontStyle, Sdl2TtfContext, Font};
use sdl2::render::{TextureCreator, Texture};
use sdl2::pixels::Color;

pub type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

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

impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
    type Args = FontDetails;
    fn load(&'l mut self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
        self.load_font(&details.path, details.size)
    }
}


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

struct GlyphCreator<'l, T> {
    font_manager: FontManager<'l>,
    texture_creator: TextureCreator<T>,
}

impl<'l, T> ResourceLoader<'l, Texture<'l>> for GlyphCreator<'l, T> {
    type Args = GlyphDetails;
    fn load(&'l mut self, details: &GlyphDetails) -> Result<Texture<'l>, String> {
        let mut f = self.font_manager.get_mut(&details.font);
        f.set_style(details.style);
        let s = f.render_char(details.character)
            .blended(Color::RGBA(255, 0, 0, 255))
            .unwrap();
        let t = self.texture_creator.create_texture_from_surface(s).unwrap();
        Ok(t)
    }
}

