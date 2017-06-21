use sdl2::video::Window;
use sdl2::render::{TextureQuery, Canvas, Texture};
use super::Context;
use types::Rect;
use std::cell::RefCell;


impl<'a, 'b> Context<'a, 'b> {
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
    pub fn image_from_file_for<F>(&self, file: String, callback: F)
    where
        F: FnOnce(&RefCell<Canvas<Window>>, &RefCell<Texture>),
    {
        let t = self.graphics.texture_manager.borrow_mut();
        let t = t.get(file);
        callback(&self.graphics.canvas, &t);
    }
}
