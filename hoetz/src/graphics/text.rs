use super::Graphics;
use super::resource_manager::{GlyphDetails, FontDetails};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::ttf::FontStyle;
use sdl2::render::TextureQuery;
use std::path::Path;
use std;
pub mod style {
    use sdl2::ttf;
    use sdl2::ttf::FontStyle;

    pub const NORMAL: FontStyle = ttf::STYLE_NORMAL;
    pub const BOLD: FontStyle = ttf::STYLE_BOLD;
    pub const ITALIC: FontStyle = ttf::STYLE_ITALIC;
    pub const UNDERLINE: FontStyle = ttf::STYLE_UNDERLINE;
    pub const STRIKETHROUGH: FontStyle = ttf::STYLE_STRIKETHROUGH;
}
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

impl<'t> Graphics<'t> {
    pub fn text<'a>(
        &mut self,
        s: String,
        font: &'static str,
        size: u16,
        x: i32,
        y: i32,
        color: Color,
        style: FontStyle,
    ) {

        let mut r = rect!(x, y, 0, 0);
        let mut g = GlyphDetails {
            character: ' ',
            style: style,
            font: FontDetails {
                path: font,
                size: size,
            },
        };
        let off = if style.contains(style::ITALIC) {
            (std::f64::consts::PI / 2.0 - 78.0 / 180.0 * std::f64::consts::PI).tan()
        } else {
            0.0
        };
        for c in s.chars() {
            if c == '\n' {
                let b = r.bottom();
                r.set_y(b);
                r.set_x(x);
            } else {
                g.character = c;
                let texture = self.glyph_manager.get(g.clone());
                let mut texture = texture.borrow_mut();
                texture.set_color_mod(color.r, color.g, color.b);
                texture.set_alpha_mod(color.a);
                let TextureQuery {
                    format: _,
                    access: _,
                    width: w,
                    height: h,
                } = texture.query();
                r.set_width(w);
                r.set_height(h);
                let _ = self.canvas.copy(&texture, Option::None, r);
                let off = (h as f64 * off) as i32;
                let rr = r.right() - off;
                r.set_x(rr);
            }
        }
    }
}
