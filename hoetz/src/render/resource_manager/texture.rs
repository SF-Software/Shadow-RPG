mod old {
    use super::{ResourceManager, ResourceLoader};
    use sdl2::image::LoadTexture;
    use sdl2::render::{TextureCreator, Texture};

    pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;

    // TextureCreator knows how to load Textures
    impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
        type Args = str;
        fn load(&'l self, path: &str) -> Result<Texture, String> {
            self.load_texture(path)
        }
    }


}

