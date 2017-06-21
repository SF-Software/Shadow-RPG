use sdl2;
pub type Color = sdl2::pixels::Color;
pub type Rect = sdl2::rect::Rect;


pub mod font {
    use sdl2;
    pub type FontStyle = sdl2::ttf::FontStyle;
    pub mod style {
        use sdl2::ttf;
        use sdl2::ttf::FontStyle;

        pub const NORMAL: FontStyle = ttf::STYLE_NORMAL;
        pub const BOLD: FontStyle = ttf::STYLE_BOLD;
        pub const ITALIC: FontStyle = ttf::STYLE_ITALIC;
        pub const UNDERLINE: FontStyle = ttf::STYLE_UNDERLINE;
        pub const STRIKETHROUGH: FontStyle = ttf::STYLE_STRIKETHROUGH;
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub struct FontDetails {
        pub size: u16,
        pub path: &'static str,
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
    pub struct GlyphDetails {
        pub character: char,
        pub style: FontStyle,
        pub font: FontDetails,
    }
}
