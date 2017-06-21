use super::Context;
use sdl2::render::{TextureQuery, Canvas, Texture};
use sdl2::pixels::Color;
use sdl2::ttf::FontStyle;
use types::Rect;
use types::font::{GlyphDetails, FontDetails, style};
use std;
use std::cell::RefCell;
use sdl2::video::Window;
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

impl<'a, 'b> Context<'a, 'b> {
    pub fn text_for<F>(
        &self,
        s: String,
        font: &'static str,
        size: u16,
        color: Color,
        style: FontStyle,
        callback: F,
    ) where
        F: FnOnce(&RefCell<Canvas<Window>>, &RefCell<Texture>),
    {
        let f = FontDetails {
            path: font,
            size: size,
        };
        let gm = self.graphics.glyph_manager.borrow();
        let t = gm.get_string(s, style, f);
        {
            let mut tt = t.borrow_mut();
            tt.set_color_mod(color.r, color.g, color.b);
            tt.set_alpha_mod(color.a);
        }

        callback(&self.graphics.canvas, &t);
    }
    pub fn text(
        &self,
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
                let texture = self.graphics.glyph_manager.borrow_mut().get(g.clone());
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
                let _ = self.graphics.canvas.borrow_mut().copy(
                    &texture,
                    Option::None,
                    r,
                );
                let off = (h as f64 * off) as i32;
                let rr = r.right() - off;
                r.set_x(rr);
            }
        }
    }
}
