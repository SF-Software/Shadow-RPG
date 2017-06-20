/* Project @ Shadow RPG
 * @ SF Software
 */


extern crate hoetz;
use hoetz::scene;
use hoetz::scene::{Command, UIInput};
use hoetz::render::Renderer;
#[derive(Clone)]
struct Model {}
fn init() -> (Model, Command) {
    (Model {}, Command::None)
}
fn update(m: &Model, i: UIInput) -> (Model, Command) {
    (m.clone(), Command::None)
}
fn view(m: &Model, r: &mut Renderer) {
    r.image_from_file(String::from("title.jpg"), 0, 0);
}
fn main() {
    hoetz::game_start(scene::new(init(), update, view), 60);

}

