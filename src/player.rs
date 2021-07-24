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
}

impl<'a> Player<'a> {
    pub fn new<T>(creator: &'a TextureCreator<T>, position: Point<i32>, size: Point<u32>, dy: i32) -> Self {
        Self {
            texture: load_texture("Assets/grass.png", &creator),
            position: position,
            size: size,
            dy: dy,
            direction: Direction::Right,
            moving: false,
            jumping: false,
        }
    }

    fn animate(&self) {
        println!("Animate!");
    }

    pub fn input_and_update(&mut self, e: &sdl2::EventPump) {
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
            self.moving = true;
            self.position.y -= self.dy * 2; // Jump
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
        /*canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(self.position.x, self.position.y, 
        self.size.x, self.size.y))?;*/
        self.animate();
        let dst_rect = Rect::new(self.position.x, self.position.y, self.size.x, self.size.y);
        canvas.copy(&self.texture, None, dst_rect)?;

        Ok(())
    }
}
