
use sdl2::render::TextureQuery;
use super::Context;
use types::Rect;

impl<'a, 'b> Context<'a, 'b> {
    pub fn image_from_file(&self, file: String, x: i32, y: i32) {
        let t = self.graphics.texture_manager.borrow_mut();
        let t = t.get(file);
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
}
