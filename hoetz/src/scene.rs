use super::graphics::Graphics;
use super::event::UIInput;

pub enum Command {
    None,
    SceneChange(BoxedScene),
}

pub type Update<M> = (fn(&M, &UIInput) -> (M, Command));
pub type ViewRenderer<M> = fn(&M, &mut Graphics);




pub trait Scene {
    fn update(&mut self, &UIInput) -> Option<BoxedScene>;
    fn render_view(&self, &mut Graphics);
}

pub type BoxedScene = Box<Scene>;

pub struct SceneEntity<M> {
    upda: Update<M>,
    view_renderer: ViewRenderer<M>,
    model: M,
}

fn process_command(c: Command) -> Option<BoxedScene> {
    match c {
        Command::None => Option::None,
        Command::SceneChange(s) => Some(s),
    }
}

impl<M> Scene for SceneEntity<M> {
    fn update(&mut self, input: &UIInput) -> Option<BoxedScene> {
        let update = self.upda;
        let (m, c) = update(&self.model, input);
        self.model = m;
        process_command(c)
    }
    fn render_view(&self, renderer: &mut Graphics) {
        let vr = self.view_renderer;
        vr(&self.model, renderer);
    }
}



pub fn new<M>(
    init: (M, Command),
    update: Update<M>,
    view_renderer: ViewRenderer<M>,
) -> Box<SceneEntity<M>> {
    let (m, c) = init;
    process_command(c);
    Box::new(SceneEntity {
        model: m,
        upda: update,
        view_renderer: view_renderer,
    })
}
