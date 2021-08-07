use crate::entity_trait::EntityDefault;
use crate::util::{load_texture, Point};

use crate::input::Input;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

const SPRITE_SIZE: u32 = 16;

pub enum Direction {
    Left,
    Right,
}

pub struct Entity<'a> {
    texture: Texture<'a>,
    pub position: Point<i32>,
    size: Point<u32>,
    direction: Direction,
    current_frame: u8,
    pub moving: bool,
    pub colliding: bool,
    _dy: u8,
}

impl<'a> Entity<'a> {
    pub fn new<T>(
        creator: &'a TextureCreator<T>,
        text_path: &'a str,
        pos: Point<i32>,
        size: Point<u32>,
        _dy: u8,
    ) -> Self {
        Self {
            texture: load_texture(text_path, creator),
            position: pos,
            size: size,
            direction: Direction::Right,
            current_frame: 0,
            moving: false,
            colliding: false,
            _dy: 5,
        }
    }

    pub fn input(&mut self, e: &mut sdl2::EventPump, input: &mut Input) {
        if e.keyboard_state().is_scancode_pressed(Scancode::A) || input.left {
            self.moving = true;
            self.direction = Direction::Left;
            self.position.x -= 5;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::D) || input.right {
            self.moving = true;
            self.direction = Direction::Right;
            self.position.x += 5;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::Space) || input.jump {
            //self.jumping = true;
            self.colliding = false;
            self.position.y -= self._dy as i32 * 2; // Jump
        }
    }

    fn sprite_direction(&self, direction: &Direction) -> i32 {
        use self::Direction::*;
        match direction {
            Left => 1,
            Right => 2,
        }
    }

    pub fn update(&mut self, counter: i32) {
        /* Update/Animate Code */
        if self.moving {
            if counter % 3 == 0 {
                // every 3 frames animate
                self.current_frame = (self.current_frame + 1) % 3;
            }
        }

        if !self.moving {
            // Set player to the idle standing sprite
            match self.direction {
                Direction::Left => {
                    self.current_frame = 0;
                }
                Direction::Right => {
                    self.current_frame = 0;
                }
            }
        }

        if !self.colliding {
            self.position.y += self._dy as i32; // Gravity
        }

        self.moving = false;
    }
}

impl EntityDefault for Entity<'_> {
    fn update(&mut self) {}

    fn render(&self, canvas: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
        let src_rect = Rect::new(
            SPRITE_SIZE as i32 * self.current_frame as i32,
            SPRITE_SIZE as i32 * self.sprite_direction(&self.direction),
            SPRITE_SIZE,
            SPRITE_SIZE,
        );
        let dst_rect = Rect::new(self.position.x, self.position.y, self.size.x, self.size.y);
        canvas.copy(&self.texture, src_rect, dst_rect)?;
        Ok(())
    }
}
