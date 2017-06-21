pub mod image;
pub mod text;

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
}
