use super::Renderer;
use super::resource_manager::{GlyphDetails, FontDetails};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::ttf::FontStyle;
use sdl2::render::TextureQuery;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

impl<'l> Renderer<'l> {
    pub fn text(&'l mut self,
                s: String,
                font: String,
                size: u16,
                x: i32,
                y: i32,
                color: Color,
                style: FontStyle) {

        let mut r = rect!(x, y, 0, 0);
        let mut g = GlyphDetails {
            character: ' ',
            style: style,
            font: FontDetails {
                path: font,
                size: size,
            },
        };
        for c in s.chars() {
            {
                if c == '\n' {
                    let b = r.bottom();
                    r.set_y(b);
                    r.set_x(x);
                } else {
                    {
                        g.character = c;
                        let texture = self.glyph_manager.get(g.clone());
                        let TextureQuery {
                            format: _,
                            access: _,
                            width: w,
                            height: h,
                        } = texture.query();
                        r.set_width(w);
                        r.set_height(h);
                        self.canvas.copy(&texture, Option::None, r);
                        let rr = r.right();
                        r.set_x(rr);
                    }

                }
            }
        }
    }
}

