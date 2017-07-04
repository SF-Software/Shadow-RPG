use super::event::UIInput;
use super::context::Context;


pub enum Command {
    None,
    SceneChange(BoxedScene),
}

pub type Init<M, A> = (fn(A) -> (M, Command));
pub type ResourceLoad<'a, M, R> = fn(&M, &Context<'a>) -> R;
pub type Update<M> = (fn(&M, &UIInput) -> (M, Command));
pub type ViewRenderer<'a, M, R> = fn(&M, &R, u32, &Context<'a>);




pub trait Scene<'a> {
    fn update(&mut self, &UIInput) -> Option<BoxedScene<'a>>;
    fn render_view(&self, &Context<'a>, u32);
    fn resource_load(&mut self, &Context<'a>);
}

pub type BoxedScene<'a> = Box<Scene<'a>>;

pub struct SceneEntity<'a, M, R> {
    upda: Update<M>,
    resource_loader: ResourceLoad<'a, M, R>,
    view_renderer: ViewRenderer<'a, M, R>,
    model: M,
    resource: Option<R>,
}

fn process_command(c: Command) -> Option<BoxedScene> {
    match c {
        Command::None => Option::None,
        Command::SceneChange(s) => Some(s),
    }
}

impl<'a, M, R> Scene<'a> for SceneEntity<'a, M, R> {
    fn resource_load(&mut self, context: &Context<'a>) {
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
    fn render_view(&self, context: &Context<'a>, frame: u32) {
        let vr = self.view_renderer;
        if let Some(ref r) = self.resource {
            vr(&self.model, &r, frame, context);
        }

    }
}



pub fn new<'a, M, A, R>(
    args: A,
    init: Init<M, A>,
    resource_loader: ResourceLoad<'a, M, R>,
    update: Update<M>,
    view_renderer: ViewRenderer<'a, M, R>,
) -> Box<SceneEntity<'a, M, R>> {
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
