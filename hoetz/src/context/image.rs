use std::rc::Rc;
use std::cell::RefCell;
use types::Rect;
use super::{Context, ResourceContext, CopyAttribute};
use sdl2::render::{TextureQuery, Texture};



impl<'b> Context<'b> {
    pub fn image_from_file_ex<F>(&self, file: String, callback: F)
    where
        F: FnOnce(CopyAttribute, u32, u32) -> CopyAttribute,
    {
        let t = self.graphics.texture_manager.borrow_mut();
        let t = t.get(file);
        self.copy_ex(&t, callback);
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
