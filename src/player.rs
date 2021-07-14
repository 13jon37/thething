use crate::entity_trait::EntityDefault;
use crate::util::Point;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::keyboard::Scancode;

pub enum Direction {
    Left,
    Right,
}

pub struct Player {
    position: Point<i32>,
    size: Point<u32>,
    dy: i32,
    pub direction: Direction,
    pub moving: bool,
}

impl Player {
    pub fn new(position: Point<i32>, size: Point<u32>, dy: i32) -> Self {
        Self {
            position: position,
            size: size,
            dy: dy,
            direction: Direction::Right,
            moving: false,
        }
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
        self.position.y += self.dy; // Gravity
        self.moving = false;
    }
}

impl EntityDefault for Player {
    // This method was pretty much replaced by the player input method
    fn update(&mut self) {}

    fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(self.position.x, self.position.y, 
                                   self.size.x, self.size.y))?;
        Ok(())
    }
}
