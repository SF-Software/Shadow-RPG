pub mod image;
pub mod text;
use sdl2::video::Window;
use sdl2::render::Canvas;
use std::cell::RefCell;
use super::graphics::Graphics;


pub struct Context<'a, 'b>
where
    'b: 'a,
{
    graphics: &'a Graphics<'b>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(graphics: &'a Graphics<'b>) -> Context<'a, 'b> {
        Self { graphics: graphics }
    }
    pub fn get_canvas<F>(&self, callback: F)
    where
        F: FnOnce(&RefCell<Canvas<Window>>),
    {
        callback(&self.graphics.canvas);
    }
}
