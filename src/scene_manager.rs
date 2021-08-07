use sdl2::{pixels::Color, render::TextureCreator};

use crate::{
    entity_manager::EntityManager, entity_trait::EntityDefault, input::Input,
    tile_manager::TileManager,
};

pub struct SceneManager<'a> {
    pub entity_manager: EntityManager<'a>,
    tile_manager: TileManager<'a>,
    counter: i32,
}

impl<'a> SceneManager<'a> {
    pub fn new<T>(creator: &'a TextureCreator<T>) -> Self {
        Self {
            entity_manager: EntityManager::new(),
            tile_manager: TileManager::new(creator),
            counter: 0,
        }
    }

    fn collision(&mut self) {
        for entity in 0..self.entity_manager.entities.len() {
            for tile in 0..self.tile_manager.tile_pos.len() {
                if self.entity_manager.entities[entity].position.x + 32
                    >= self.tile_manager.tile_pos[tile].x
                    && self.entity_manager.entities[entity].position.x
                        <= self.tile_manager.tile_pos[tile].x + 32
                {
                    if self.entity_manager.entities[entity].position.y + 64
                        >= self.tile_manager.tile_pos[tile].y
                        && self.entity_manager.entities[entity].position.y
                            <= self.tile_manager.tile_pos[tile].y + 64
                    {
                        self.entity_manager.entities[entity].colliding = true;
                        self.entity_manager.entities[entity].position.y =
                            self.tile_manager.tile_pos[tile].y - 64;
                    }
                } else {
                    self.entity_manager.entities[entity].colliding = false;
                }
            }
        }
    }

    pub fn update_and_render(
        &mut self,
        e: &mut sdl2::EventPump,
        canvas: &mut sdl2::render::WindowCanvas,
        input: &mut Input,
    ) -> Result<(), String> {
        self.entity_manager.update(e, input, self.counter);
        self.collision();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        self.tile_manager.render_level(canvas)?;
        self.entity_manager.render(canvas)?;

        canvas.present();
        self.counter += 1;
        Ok(())
    }
}
