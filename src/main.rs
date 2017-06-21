/* Project @ Shadow RPG
 * @ SF Software
 */

extern crate hoetz;
use hoetz::scene;
use hoetz::scene::Command;
use hoetz::event::UIInput;
use hoetz::graphics::Graphics;
use hoetz::graphics::text::style;
macro_rules! color_rgba(
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        hoetz::graphics::Color::RGBA($r as u8, $g as u8, $b as u8, $a as u8)
    )
);

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
       hoetz::graphics::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Clone)]
struct Model {
    y: i32,
    uptowards: bool,
}
fn init() -> (Model, Command) {
    (
        Model {
            y: 500,
            uptowards: false,
        },
        Command::None,
    )
}
fn update(m: &Model, i: &UIInput) -> (Model, Command) {
    let mut ny = m.y;
    let mut up = m.uptowards;
    if up {
        ny -= 2;
    } else {
        ny += 2;
    }
    if ny < 450 || ny > 550 {
        up = !up;
    }
    (
        Model {
            y: ny,
            uptowards: up,
        },
        Command::None,
    )
}

fn view(m: &Model, r: &mut Graphics) {
    let font = "NotoSansCJKtc-Regular.otf";
    r.image_from_file(String::from("title.jpg"), 0, 0);
    r.text(
        String::from("Start"),
        font,
        32,
        80,
        m.y,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );
    r.text(
        String::from("Load"),
        font,
        32,
        280,
        500,
        color_rgba!(255, 255, 255, 128),
        style::NORMAL,
    );
    r.text(
        String::from("Setting"),
        font,
        32,
        480,
        500,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );
    r.text(
        String::from("Exit"),
        font,
        32,
        680,
        500,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );

}
fn main() {
    hoetz::game_start(
        800,
        600,
        "The Dreamer".to_owned(),
        scene::new(init(), update, view),
        60,
    );

}
