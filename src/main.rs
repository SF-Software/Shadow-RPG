/* Project @ Shadow RPG
 * @ SF Software
 */

extern crate hoetz;
use hoetz::scene;
use hoetz::scene::Command;
use hoetz::event::UIInput;
use hoetz::context::Context;
use hoetz::helper::query_texture;
use hoetz::types::font::style;


macro_rules! color_rgba(
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        hoetz::types::Color::RGBA($r as u8, $g as u8, $b as u8, $a as u8)
    )
);

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
       hoetz::types::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Clone)]
struct Model {
    y: i32,
    uptowards: bool,
}

fn init() -> (Model, (), Command) {
    (
        Model {
            y: 500,
            uptowards: false,
        },
        (),
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

fn view(m: &Model, r: &(), c: &Context) {
    let font = "NotoSansCJKtc-Regular.otf";
    c.image_from_file("title.jpg".to_owned(), 0, 0);
    c.image_from_file_for("title.jpg".to_owned(), |canvas, texture| {
        let texture = texture.borrow();
        let (w, h) = query_texture(&texture);
        canvas.borrow_mut().copy(&texture, None, rect!(10, 0, w, h));
    });
    c.text(
        "Start".to_owned(),
        font,
        32,
        80,
        m.y,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );
    c.text(
        "Load".to_owned(),
        font,
        32,
        280,
        500,
        color_rgba!(255, 255, 255, 128),
        style::NORMAL,
    );
    c.text(
        "Settings".to_owned(),
        font,
        32,
        480,
        500,
        color_rgba!(255, 255, 255, 255),
        style::NORMAL,
    );
    c.text(
        "Exit".to_owned(),
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
        scene::new(init, update, view),
        60,
    );

}
