/* Project @ Shadow RPG
 * @ SF Software
 */

extern crate hoetz;
use hoetz::scene;
use hoetz::scene::{Command, UIInput};
use hoetz::render::Renderer;
use hoetz::render::text::style;
macro_rules! color_rgba(
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        hoetz::render::Color::RGBA($r as u8, $g as u8, $b as u8, $a as u8)
    )
);

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
       hoetz::render::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Clone)]
struct Model {}
fn init() -> (Model, Command) {
    (Model {}, Command::None)
}
fn update(m: &Model, i: UIInput) -> (Model, Command) {
    (m.clone(), Command::None)
}

/*
 pub fn text(
        &mut self,
        s: String,
        font: String,
        size: u16,
        x: i32,
        y: i32,
        color: Color,
        style: FontStyle,
    )
*/
fn view(m: &Model, r: &mut Renderer) {
    r.image_from_file(String::from("title.jpg"), 0, 0);
    r.text(
        String::from("Start"),
        String::from("arial.ttf"),
        32,
        80,
        350,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );
    r.text(
        String::from("Load"),
        String::from("arial.ttf"),
        32,
        180,
        350,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );
    r.text(
        String::from("Setting"),
        String::from("arial.ttf"),
        32,
        320,
        350,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );
    r.text(
        String::from("Exit"),
        String::from("arial.ttf"),
        32,
        480,
        350,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );

}
fn main() {
    hoetz::game_start(scene::new(init(), update, view), 60);

}
