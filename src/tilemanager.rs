use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::rect::Rect;
use crate::util::load_texture;

const TILE_SIZE: u32 = 16;
const LEVEL_WIDTH: usize = 5;
const LEVEL_HEIGHT: usize = 5;

/*
struct Tile<'a> {
    tile: Texture<'a>,
    src_rect: Rect,
    dst_rect: Rect,
}

impl<'a> Tile<'a> {
    fn render_tile(&self,)
}
*/

pub struct TileManager<'a> {
    level: [[i32; LEVEL_HEIGHT]; LEVEL_WIDTH],
    tile_set: Texture<'a>,
}

impl<'a> TileManager<'a> {
    pub fn new<T>(creator:&'a TextureCreator<T>) -> Self{
        Self {
            level: [[0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0],
                    [0, 0, 1, 0, 0],
                    [1, 1, 0, 1, 1]],
            tile_set: load_texture("Assets/fantasy_tiles.png",
                      &creator),
        }
    }

    pub fn render_level(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for y in 0..LEVEL_HEIGHT {
            for x in 0..LEVEL_WIDTH {
                match self.level[y][x] {
                    1 => {
                        let src_rect = Rect::new(120, 128, TILE_SIZE, TILE_SIZE);
                        let dst_rect = Rect::new(x as i32 * src_rect.w,
                                                 y as i32 * src_rect.h,
                                                 src_rect.w as u32,
                                                 src_rect.h as u32);
                        canvas.copy(&self.tile_set, src_rect, dst_rect)?;
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
