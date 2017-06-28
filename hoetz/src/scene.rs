use super::event::UIInput;
use super::context::{Context, ResourceContext};


pub enum Command {
    None,
    SceneChange(BoxedScene),
}

pub type Init<M, A> = (fn(A) -> (M, Command));
pub type ResourceLoad<M, R> = fn(&M, &ResourceContext) -> R;
pub type Update<M> = (fn(&M, &UIInput) -> (M, Command));
pub type ViewRenderer<M, R> = fn(&M, &R, u32, &Context);




pub trait Scene {
    fn update(&mut self, &UIInput) -> Option<BoxedScene>;
    fn render_view(&self, &Context, u32);
    fn resource_load(&mut self, &ResourceContext);
}

pub type BoxedScene = Box<Scene>;

pub struct SceneEntity<M, R> {
    upda: Update<M>,
    resource_loader: ResourceLoad<M, R>,
    view_renderer: ViewRenderer<M, R>,
    model: M,
    resource: Option<R>,
}

fn process_command(c: Command) -> Option<BoxedScene> {
    match c {
        Command::None => Option::None,
        Command::SceneChange(s) => Some(s),
    }
}

impl<M, R> Scene for SceneEntity<M, R> {
    fn resource_load(&mut self, context: &ResourceContext) {
        if let None = self.resource {
            let rld = self.resource_loader;
            self.resource = Some(rld(&self.model, context));
        }
    }
    fn update(&mut self, input: &UIInput) -> Option<BoxedScene> {
        let update = self.upda;
        let (m, c) = update(&self.model, input);
        self.model = m;
        process_command(c)
    }
    fn render_view(&self, context: &Context, frame: u32) {
        let vr = self.view_renderer;
        if let Some(ref r) = self.resource {
            vr(&self.model, &r, frame, context);
        }

    }
}



pub fn new<M, A, R>(
    args: A,
    init: Init<M, A>,
    resource_loader: ResourceLoad<M, R>,
    update: Update<M>,
    view_renderer: ViewRenderer<M, R>,
) -> Box<SceneEntity<M, R>> {
    let (m, c) = init(args);
    process_command(c);
    Box::new(SceneEntity {
        model: m,
        resource: None,
        resource_loader: resource_loader,
        upda: update,
        view_renderer: view_renderer,
    })
}
