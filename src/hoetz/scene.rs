pub trait Input {}

pub trait Model {}
pub enum View {
    Empty,
}

pub enum Command {
    None,
    SceneChange(Box<Scene>),
}

pub struct Update<M, I>(fn(&M, I) -> (M, Command))
    where M: Model,
          I: Input;
pub struct ViewGenerator<M>(fn(M) -> View) where M: Model;


pub struct UIInput {}
impl Input for UIInput {}

pub trait Scene {
    fn update(&mut self, UIInput) -> Option<Box<Scene>>;
    fn gen_view(&mut self) -> View;
}

pub struct SceneEntity<M>
    where M: Model
{
    upda: Update<M, UIInput>,
    view_gen: ViewGenerator<M>,
    model: M,
}

fn process_command(c: Command) -> Option<Box<Scene>> {
    match c {
        Command::None => Option::None,
        Command::SceneChange(s) => Some(s),
    }
}

impl<M> Scene for SceneEntity<M>
    where M: Model
{
    fn update(&mut self, input: UIInput) -> Option<Box<Scene>> {
        let Update(update) = self.upda;
        let (m, c) = update(&self.model, input);
        self.model = m;
        process_command(c)
    }
    fn gen_view(&mut self) -> View {
        View::Empty
    }
}



pub fn start<M>(init: (M, Command),
                update: Update<M, UIInput>,
                view_gen: ViewGenerator<M>)
                -> Box<SceneEntity<M>>
    where M: Model
{
    let (m, c) = init;
    process_command(c);
    Box::new(SceneEntity {
                 model: m,
                 upda: update,
                 view_gen: view_gen,
             })
}

