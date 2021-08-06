use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};

use crate::util::load_texture;
use crate::util::Point;

const TILE_SIZE: u32 = 64;
const LEVEL_WIDTH: usize = 5;
const LEVEL_HEIGHT: usize = 5;

const TILE_EMPTY: u8 = 0;
const TILE_GRASS: u8 = 1;

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
    pub level: [[u8; LEVEL_HEIGHT]; LEVEL_WIDTH],
    tile_set: Texture<'a>,
    dst_rect: Rect,
    pub tile_pos: Vec<Point<i32>>,
}

impl<'a> TileManager<'a> {
    pub fn new<T>(creator: &'a TextureCreator<T>) -> Self {
        Self {
            level: [
                [TILE_EMPTY, TILE_EMPTY, TILE_EMPTY, TILE_EMPTY, TILE_EMPTY],
                [TILE_EMPTY, TILE_EMPTY, TILE_EMPTY, TILE_EMPTY, TILE_EMPTY],
                [TILE_EMPTY, TILE_EMPTY, TILE_GRASS, TILE_EMPTY, TILE_EMPTY],
                [TILE_EMPTY, TILE_EMPTY, TILE_GRASS, TILE_EMPTY, TILE_EMPTY],
                [TILE_GRASS, TILE_GRASS, TILE_EMPTY, TILE_GRASS, TILE_GRASS],
            ],
            tile_set: load_texture("Assets/grass.png", &creator),
            dst_rect: Rect::new(0, 0, 0, 0),
            tile_pos: Vec::new(),
        }
    }
    // Tile method needs tile coords too
    fn render_tile(
        &mut self,
        canvas: &mut WindowCanvas,
        coordinates: Point<i32>,
    ) -> Result<(), String> {
        let src_rect = Rect::new(0, 0, TILE_SIZE, TILE_SIZE);
        self.dst_rect = Rect::new(
            coordinates.x as i32 * src_rect.w,
            coordinates.y as i32 * src_rect.h,
            src_rect.w as u32,
            src_rect.h as u32,
        );
        self.tile_pos
            .push(Point::new(self.dst_rect.x, self.dst_rect.y));

        canvas.copy(&self.tile_set, src_rect, self.dst_rect)?;

        Ok(())
    }

    pub fn render_level(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for y in 0..LEVEL_HEIGHT {
            for x in 0..LEVEL_WIDTH {
                match self.level[y][x] {
                    TILE_GRASS => {
                        self.render_tile(canvas, Point::new(x as i32, y as i32))?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
