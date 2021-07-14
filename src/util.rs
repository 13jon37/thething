use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;

/*
 * Generic Vector2 Type
 */

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

/*
 * SDL load texture general use function
 */

pub fn load_texture<'a, T>(path: &'a str, creator: &'a TextureCreator<T>) -> Texture<'a> {
    return creator.load_texture(path).unwrap();
}
