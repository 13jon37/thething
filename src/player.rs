use crate::entity_trait::EntityDefault;
use crate::util::{load_texture, Point};

use crate::input::Input;
use sdl2::keyboard::Scancode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};

const SPRITE_SIZE: u32 = 16;

pub enum Direction {
    Left,
    Right,
}

pub struct Player<'a> {
    texture: Texture<'a>,
    position: Point<i32>,
    size: Point<u32>,
    dy: i32, // Gravity Y
    direction: Direction,
    moving: bool,
    jumping: bool,
    current_frame: i32,
}

impl<'a> Player<'a> {
    pub fn new<T>(
        creator: &'a TextureCreator<T>,
        position: Point<i32>,
        size: Point<u32>,
        dy: i32,
    ) -> Self {
        Self {
            texture: load_texture("Assets/player.png", &creator),
            position: position,
            size: size,
            dy: dy,
            direction: Direction::Right,
            moving: false,
            jumping: false,
            current_frame: 0,
        }
    }

    fn sprite_direction(&self, direction: &Direction) -> i32 {
        use self::Direction::*;
        match direction {
            Left => 1,
            Right => 2,
        }
    }

    pub fn input_and_update(&mut self, e: &mut sdl2::EventPump, input: &mut Input, counter: i32) {
        /* Input Code - checks if either
        controller or keyboard button is pressed. */
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
            self.jumping = true;
            self.position.y -= self.dy * 2; // Jump
        }

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
        self.position.y += self.dy; // Gravity
        self.moving = false;
        self.jumping = false;
    }
}

impl<'a> EntityDefault for Player<'a> {
    // This method is replaced by input_and_update()
    fn update(&mut self) {}

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let src_rect = Rect::new(
            SPRITE_SIZE as i32 * self.current_frame,
            SPRITE_SIZE as i32 * self.sprite_direction(&self.direction),
            SPRITE_SIZE,
            SPRITE_SIZE,
        );
        let dst_rect = Rect::new(self.position.x, self.position.y, self.size.x, self.size.y);
        canvas.copy(&self.texture, src_rect, dst_rect)?;

        Ok(())
    }
}
