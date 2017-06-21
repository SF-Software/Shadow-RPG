use super::context::Context;
use super::event::UIInput;

pub enum Command {
    None,
    SceneChange(BoxedScene),
}

pub type Init<M, R> = (fn() -> (M, R, Command));
pub type Update<M> = (fn(&M, &UIInput) -> (M, Command));
pub type ViewRenderer<M, R> = fn(&M, &R, &Context);




pub trait Scene {
    fn update(&mut self, &UIInput) -> Option<BoxedScene>;
    fn render_view(&self, &Context);
}

pub type BoxedScene = Box<Scene>;

pub struct SceneEntity<M, R> {
    upda: Update<M>,
    view_renderer: ViewRenderer<M, R>,
    model: M,
    resource: R,
}

fn process_command(c: Command) -> Option<BoxedScene> {
    match c {
        Command::None => Option::None,
        Command::SceneChange(s) => Some(s),
    }
}

impl<M, R> Scene for SceneEntity<M, R> {
    fn update(&mut self, input: &UIInput) -> Option<BoxedScene> {
        let update = self.upda;
        let (m, c) = update(&self.model, input);
        self.model = m;
        process_command(c)
    }
    fn render_view(&self, context: &Context) {
        let vr = self.view_renderer;
        vr(&self.model, &self.resource, context);
    }
}



pub fn new<M, R>(
    init: Init<M, R>,
    update: Update<M>,
    view_renderer: ViewRenderer<M, R>,
) -> Box<SceneEntity<M, R>> {
    let (m, r, c) = init();
    process_command(c);
    Box::new(SceneEntity {
        model: m,
        resource: r,
        upda: update,
        view_renderer: view_renderer,
    })
}
