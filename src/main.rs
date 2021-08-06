extern crate sdl2;

mod entity_trait;
mod input;
mod player;
mod tilemanager;
mod util;

use crate::entity_trait::EntityDefault;
use crate::input::Input;
use crate::player::Player;
use crate::tilemanager::TileManager;
use crate::util::Point;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::pixels::Color;
use std::time::Duration;

fn collision(player: &mut Player, tiles: &TileManager) {
    for i in 0..tiles.tile_pos.len() {
        if player.position.x + 32 >= tiles.tile_pos[i].x
            && player.position.x <= tiles.tile_pos[i].x + 32
        {
            if player.position.y + 64 >= tiles.tile_pos[i].y
                && player.position.y <= tiles.tile_pos[i].y + 64
            {
                player.colliding = true;
                player.position.y = tiles.tile_pos[i].y - 64;
            }
        } else {
            player.colliding = false;
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let game_controller_subsytem = sdl_context.game_controller()?;
    let available = game_controller_subsytem
        .num_joysticks()
        .map_err(|e| format!("Can't enumerate joysicks: {}", e))?;

    // From rust lang sdl2 example docs
    let _controller = (0..available).find_map(|id| {
        if !game_controller_subsytem.is_game_controller(id) {
            println!("{} is not a game controller", id);
            return None;
        }

        println!("Attempting to open controller {}", id);
        match game_controller_subsytem.open(id) {
            Ok(c) => {
                // Managed to find game controller
                println!("Success: {}", c.name());
                Some(c)
            }
            Err(e) => {
                println!("Failed: {:?}", e);
                None
            }
        }
    });

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Thething", 800, 600)
        .position_centered()
        .build()
        .expect("failed to create window.");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("failed to create canvas.");

    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    let mut game_input = Input::new();

    let mut player = Player::new(
        &texture_creator,
        Point::new(100, 100), // Position
        Point::new(64, 64),   // Size
        5,                    // Speed
    );
    let mut tilemanager = TileManager::new(&texture_creator);

    let mut counter = 0; // In order to time animation for player

    // Handle events
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                // Direction move bool is true if button is down
                Event::ControllerButtonDown { button, .. } => {
                    println!("Button: {:?}", button);
                    match button {
                        sdl2::controller::Button::A => {
                            game_input.jump = true;
                        }
                        sdl2::controller::Button::DPadLeft => {
                            game_input.left = true;
                        }
                        sdl2::controller::Button::DPadRight => {
                            game_input.right = true;
                        }
                        _ => {}
                    }
                }
                Event::ControllerButtonUp { button, .. } => {
                    println!("Button: {:?}", button);
                    match button {
                        sdl2::controller::Button::A => {
                            game_input.jump = false;
                        }
                        sdl2::controller::Button::DPadLeft => {
                            game_input.left = false;
                        }
                        sdl2::controller::Button::DPadRight => {
                            game_input.right = false;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        // Update
        player.input_and_update(&mut event_pump, &mut game_input, counter);
        collision(&mut player, &tilemanager);

        // Render
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        player.render(&mut canvas)?;
        tilemanager.render_level(&mut canvas)?;

        canvas.present();

        counter += 1;
        // Limit fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
