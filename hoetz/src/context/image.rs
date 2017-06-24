use std::rc::Rc;
use std::cell::RefCell;
use types::{Rect, Color, Point};
use super::{Context, ResourceContext};
use sdl2::render::{TextureQuery, Texture};

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

impl<'a, 'b> Context<'a, 'b> {
    pub fn image_from_file_ex<F>(&self, file: String, callback: F)
    where
        F: FnOnce(CopyAttribute, u32, u32) -> CopyAttribute,
    {
        let t = self.graphics.texture_manager.borrow_mut();
        let t = t.get(file);
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

    pub fn image_from_file(&self, file: String, x: i32, y: i32) {
        let t = self.graphics.texture_manager.borrow_mut();
        let t = t.get(file);
        let t = t.borrow();

        let TextureQuery {
            access: _,
            format: _,
            width: w,
            height: h,
        } = t.query();
        let _ = self.graphics.canvas.borrow_mut().copy(
            &t,
            None,
            Rect::new(
                x as i32,
                y as i32,
                w as u32,
                h as u32,
            ),
        );
    }
    pub fn get_image_from_file(&self, file: String) -> Rc<RefCell<Texture<'b>>> {
        let t = self.graphics.texture_manager.borrow_mut();
        let t = t.get(file);
        t
    }
}
impl<'a, 'b> ResourceContext<'a, 'b> {
    pub fn image_from_file(&self, file: String) -> Rc<RefCell<Texture<'b>>> {
        let t = self.graphics.texture_manager.borrow_mut();
        let t = t.get(file);
        t
    }
}
