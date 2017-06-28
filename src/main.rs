/* Project @ Shadow RPG
 * @ SF Software
 */

extern crate hoetz;
use hoetz::scene;
use hoetz::scene::Command;
use hoetz::event::UIInput;
use hoetz::context::{Context, ResourceContext, RcTexture};
use hoetz::helper::query_texture;
use hoetz::types::font::style;
use hoetz::types::KeyCode;
//use hoetz::context::image::CopyAttribute;

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
    select_index: i32,
}

fn init(args: ()) -> (Model, Command) {
    (Model { select_index: 0 }, Command::None)
}
fn resource_loader(m: &Model, context: &ResourceContext)->(){

}
fn r2esource_loader<'a>(m: &Model, context: &'a ResourceContext<'a, 'a>) -> Vec<RcTexture<'a>> {
    let font = "NotoSansCJKtc-Regular.otf";
    vec!["Start", "Load", "Settings", "Exit"]
        .into_iter()
        .map(String::from)
        .map(|s| {
            context.get_text(s, font, 32, color_rgba!(255, 255, 255, 255), style::NORMAL)
        })
        .collect()
}
fn update(m: &Model, i: &UIInput) -> (Model, Command) {
    let mut index = m.select_index;
    if i.keyboard.trigger(KeyCode::Right) {
        index += 1;
    }
    if i.keyboard.trigger(KeyCode::Left) {
        index -= 1;
    }
    index += 4;
    index %= 4;

    (Model { select_index: index }, Command::None)
}

fn view(m: &Model, r: &(), frame: u32, c: &Context) {

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
    let y = if frame % 60 < 30 {
        (500 + frame % 30 - 15) as i32
    } else {
        (500 - frame % 30 + 15) as i32
    };
    let mut ys: [i32; 4] = [500, 500, 500, 500];
    ys[m.select_index as usize] = y;
    /*r.iter().enumerate().fold(80, |x, (i, ref v)| {
        c.copy_ex(v, |c, w, h| c.pos(x, 500));
        x + 200
    });*/

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
