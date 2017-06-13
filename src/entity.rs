
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Context;
use piston::input::*;

pub trait Input {}

pub trait State {}
pub trait RendererState {}

pub enum CurrentState {
    Running,
    Dead,
}

pub trait Entity<S, R, I>
    where S: State,
          R: RendererState,
          I: Input
{
    fn process(&mut self, i: I);
    fn renderer(&mut self, &mut GlGraphics, &Context, &RenderArgs);
    fn update(s: &S, i: &I) -> (S, R, CurrentState);
}

