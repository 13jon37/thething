use crate::entity_trait::EntityDefault;
use crate::util::{Point, load_texture};

use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::rect::Rect;
use sdl2::keyboard::Scancode;

pub enum Direction {
    Left,
    Right,
}

pub struct Player<'a> {
    texture: Texture<'a>,
    position: Point<i32>,
    size: Point<u32>,
    dy: i32,
    direction: Direction,
    moving: bool,
    jumping: bool,
    current_frame: i32,
}

impl<'a> Player<'a> {
    pub fn new<T>(creator: &'a TextureCreator<T>, position: Point<i32>, size: Point<u32>, dy: i32) -> Self {
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

    pub fn input_and_update(&mut self, e: &sdl2::EventPump) {
        /* Input Code */
        if e.keyboard_state().is_scancode_pressed(Scancode::A) {
            self.moving = true;
            self.direction = Direction::Left;
            self.position.x -= 5;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::D) {
            self.moving = true;
            self.direction = Direction::Right;
            self.position.x += 5;
        }
        if e.keyboard_state().is_scancode_pressed(Scancode::Space) {
            self.jumping = true;
            //self.moving = true;
            self.position.y -= self.dy * 2; // Jump
        }

         /* Update/Animate Code */
        if self.moving {
            self.current_frame = (self.current_frame + 1) % 3;
        }
        if !self.moving {
            // Set player to the idle standing sprite
            match self.direction {
                Direction::Left  => {
                    self.current_frame = 0;
                },
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
    // This method was pretty much replaced by the player input method
    fn update(&mut self) {}

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let src_rect = Rect::new(
             0 + 16 * self.current_frame,
             0 + 16 * self.sprite_direction(&self.direction),
            16, 16
        );
        let dst_rect = Rect::new(self.position.x, 
                                      self.position.y,
                                self.size.x,
                               self.size.y);
        canvas.copy(&self.texture, src_rect, dst_rect)?;

        Ok(())
    }
}
