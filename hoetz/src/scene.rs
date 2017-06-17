use super::render::Renderer;

pub trait Input {}

pub trait Model {}

pub enum Command {
    None,
    SceneChange(BoxedScene),
}

pub struct Update<M, I>(fn(&M, I) -> (M, Command))
    where M: Model,
          I: Input;
pub struct ViewRenderer<M>(fn(&M, &Renderer)) where M: Model;


pub struct UIInput {}
impl Input for UIInput {}

pub trait Scene {
    fn update(&mut self, UIInput) -> Option<BoxedScene>;
    fn render_view(&self, &mut Renderer);
}

pub type BoxedScene = Box<Scene>;

pub struct SceneEntity<M>
    where M: Model
{
    upda: Update<M, UIInput>,
    view_renderer: ViewRenderer<M>,
    model: M,
}

fn process_command(c: Command) -> Option<BoxedScene> {
    match c {
        Command::None => Option::None,
        Command::SceneChange(s) => Some(s),
    }
}

impl<M> Scene for SceneEntity<M>
    where M: Model
{
    fn update(&mut self, input: UIInput) -> Option<BoxedScene> {
        let Update(update) = self.upda;
        let (m, c) = update(&self.model, input);
        self.model = m;
        process_command(c)
    }
    fn render_view(&self, renderer: &mut Renderer) {
        let ViewRenderer(vr) = self.view_renderer;
        vr(&self.model, renderer);
    }
}



pub fn start<M>(init: (M, Command),
                update: Update<M, UIInput>,
                view_renderer: ViewRenderer<M>)
                -> Box<SceneEntity<M>>
    where M: Model
{
    let (m, c) = init;
    process_command(c);
    Box::new(SceneEntity {
                 model: m,
                 upda: update,
                 view_renderer: view_renderer,
             })
}

