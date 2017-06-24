/* Project @ Shadow RPG
 * @ SF Software
 */

extern crate hoetz;
use hoetz::scene;
use hoetz::scene::Command;
use hoetz::event::UIInput;
use hoetz::context::{Context, ResourceContext};
use hoetz::helper::query_texture;
use hoetz::types::font::style;
use hoetz::context::image::CopyAttribute;

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

fn init(args: ()) -> (Model, Command) {
    (
        Model {
            y: 500,
            uptowards: false,
        },
        Command::None,
    )
}
fn resource_loader(m: &Model, context: &ResourceContext) -> () {
    ()
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

    c.get_canvas(|canvas| {
        let t = c.get_image_from_file("title.jpg".to_owned());
        let t = t.borrow();
        let (w, h) = query_texture(&t);
        canvas.borrow_mut().copy(&t, None, rect!(10, 0, w, h));
    });
    c.image_from_file_ex("title.jpg".to_owned(), |c, w, h| {
        c.pos(20, 20)
            .dst_size((w as f64 * 0.5) as u32, h)
            .angle(3.14 / 4.0)
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
        scene::new((), init, resource_loader, update, view),
        60,
    );

}
