use sdl2::render::{TextureQuery, Texture};
pub fn query_texture(t: &Texture) -> (u32, u32) {
    let TextureQuery {
        format: _,
        access: _,
        width: w,
        height: h,
    } = t.query();
    (w, h)
}
