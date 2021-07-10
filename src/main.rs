extern crate sdl2;

mod entity_trait;
mod util;
mod player;

use crate::entity_trait::EntityDefault;
use crate::util::Point;
use crate::player::{Player};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Thething", 800, 600)
        .position_centered()
        .build()
        .expect("failed to create window");

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .expect("failed to create canvas");

    let mut event_pump = sdl_context.event_pump()?;

    let mut player = Player::new(Point::new(250, 250), Point::new(64, 64), 5);

    const LEVEL_WIDTH: usize = 5;
    const LEVEL_HEIGHT: usize = 5;

    //let mut level = vec![vec![0; LEVEL_WIDTH]; LEVEL_HEIGHT];
    let level: [[i32; LEVEL_HEIGHT]; LEVEL_WIDTH] =
        [[0, 0, 0, 0, 0],
         [0, 0, 0, 0, 0],
         [0, 1, 0, 0, 0],
         [0, 1, 0, 0, 0],
         [0, 1, 0, 1, 1]];

    println!("{:?}", level);

    //level[0][4] = 1;
    //level[0][2] = 1;
    //level[0][1] = 1;

    // Handle events
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                _ => {}
            }
        }
        // Update
        player.input(&event_pump);

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        player.render(&mut canvas)?;
        for y in 0..LEVEL_HEIGHT {
            for x in 0..LEVEL_WIDTH {
                match level[y][x] {
                    1 => {
                        canvas.set_draw_color(Color::RGB(255, 0, 255));
                        canvas.fill_rect(Rect::new((y * 64) as i32,
                                                   (x * 64) as i32,
                                                   64, 64))?;
                    },
                    _ => {}
                }
            }
        }

        canvas.present();

        // Limit fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
