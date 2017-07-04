pub mod image;
pub mod text;

use std::rc::Rc;
use std::cell::RefCell;
use sdl2::video::Window;
use sdl2::render::Canvas;
use super::graphics::Graphics;
use types::{Rect, Color, Point};
use sdl2::render::{TextureQuery, Texture};

pub type RcTexture<'a> = Rc<RefCell<Texture<'a>>>;
pub struct CopyAttribute {
    x: i32,
    y: i32,
    src: Option<Rect>,
    dst: Option<(u32, u32)>,
    ang: f64,
    cen: Option<(i32, i32)>,
    flip_horizontal: bool,
    flip_vertical: bool,
    col: Option<Color>,
}
impl CopyAttribute {
    pub fn new() -> CopyAttribute {
        Self {
            x: 0,
            y: 0,
            src: None,
            dst: None,
            ang: 0.0,
            cen: None,
            col: None,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }
    pub fn pos(self, x: i32, y: i32) -> CopyAttribute {
        Self { x: x, y: y, ..self }
    }
    pub fn angle(self, ang: f64) -> CopyAttribute {
        Self { ang: ang, ..self }
    }
    pub fn flip_horizontal(self, fh: bool) -> CopyAttribute {
        Self {
            flip_horizontal: fh,
            ..self
        }
    }
    pub fn flip_vertical(self, fv: bool) -> CopyAttribute {
        Self {
            flip_vertical: fv,
            ..self
        }
    }
    pub fn src_rect(self, src: Rect) -> CopyAttribute {
        Self {
            src: Some(src),
            ..self
        }
    }
    pub fn dst_size(self, w: u32, h: u32) -> CopyAttribute {
        Self {
            dst: Some((w, h)),
            ..self
        }
    }
    pub fn center(self, cen: (i32, i32)) -> CopyAttribute {
        Self {
            cen: Some(cen),
            ..self
        }
    }
    pub fn color(self, col: Color) -> CopyAttribute {
        Self {
            col: Some(col),
            ..self
        }
    }
}

fn into_point(s: Option<(i32, i32)>) -> Option<Point> {
    s.map(|se| {
        let (x, y) = se;
        Point::new(x, y)
    })
}

pub struct Context<'b> {
    graphics: Rc<Graphics<'b>>,
}

impl<'b> Context<'b> {
    pub fn new(graphics: Rc<Graphics<'b>>) -> Context<'b> {
        Self { graphics: graphics }
    }
    pub fn get_canvas<F>(&self, callback: F)
    where
        F: FnOnce(&RefCell<Canvas<Window>>),
    {
        callback(&self.graphics.canvas);
    }
    pub fn copy_ex<F>(&self, t: &Rc<RefCell<Texture>>, callback: F)
    where
        F: FnOnce(CopyAttribute, u32, u32) -> CopyAttribute,
    {
        let mut t = t.borrow_mut();
        let TextureQuery {
            access: _,
            format: _,
            width: w,
            height: h,
        } = t.query();
        let CopyAttribute {
            x,
            y,
            src,
            dst,
            ang,
            cen,
            col,
            flip_horizontal,
            flip_vertical,
        } = callback(CopyAttribute::new(), w, h);
        if let Some(c) = col {
            t.set_color_mod(c.r, c.g, c.b);
            t.set_alpha_mod(c.a);
        }
        let _ = self.graphics.canvas.borrow_mut().copy_ex(
            &t,
            src,
            if let Some((w, h)) = dst {
                Rect::new(x as i32, y as i32, w as u32, h as u32)
            } else {
                Rect::new(x as i32, y as i32, w as u32, h as u32)
            },
            ang,
            into_point(cen),
            flip_horizontal,
            flip_vertical,
        );
    }
}

pub struct ResourceContext<'a, 'b>
where
    'b: 'a,
{
    graphics: &'a Graphics<'b>,
}

impl<'a, 'b> ResourceContext<'a, 'b> {
    pub fn new(graphics: &'a Graphics<'b>) -> ResourceContext<'a, 'b> {
        Self { graphics: graphics }
    }
}
