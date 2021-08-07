extern crate sdl2;

mod entity;
mod entity_manager;
mod entity_trait;
mod input;
mod scene_manager;
mod tile_manager;
mod util;

use crate::{entity::Entity, input::Input, scene_manager::SceneManager, util::Point};

use sdl2::event::Event;
use sdl2::image::{self, InitFlag};

use std::time::Duration;

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

    // Instaniate scene manager
    let mut scene_manager = SceneManager::new(&texture_creator);

    // Instansiate player
    let player = Entity::new(
        &texture_creator,
        "Assets/player.png",
        Point::new(50, 50),
        Point::new(64, 64),
        5,
    );

    // Add an entity to the scene
    scene_manager.entity_manager.create(player);

    // Add random other entity to the screen
    scene_manager.entity_manager.create(Entity::new(
        &texture_creator,
        "Assets/player.png",
        Point::new(150, 50),
        Point::new(64, 64),
        5,
    ));

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

        scene_manager.update_and_render(&mut event_pump, &mut canvas, &mut game_input)?;

        // Limit fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
