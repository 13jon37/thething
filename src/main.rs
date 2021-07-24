extern crate sdl2;

mod entity_trait;
mod util;
mod player;
mod tilemanager;

use crate::entity_trait::EntityDefault;
use crate::util::Point;
use crate::player::Player;
use crate::tilemanager::TileManager;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Thething", 800, 600)
        .position_centered()
        .build()
        .expect("failed to create window");

    let mut canvas = window.into_canvas()
        .accelerated()
        .build()
        .expect("failed to create canvas");

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    let mut player = Player::new(&texture_creator,
                                 Point::new(100, 100), // Position
                                 Point::new(64, 64),   // Size
                                 5);                   // Speed

    let tilemanager = TileManager::new(&texture_creator);

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
        player.input_and_update(&event_pump);

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        player.render(&mut canvas)?;
        tilemanager.render_level(&mut canvas)?;

        canvas.present();

        // Limit fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
