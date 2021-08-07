use crate::entity::Entity;
use crate::entity_trait::EntityDefault;
use crate::input::Input;

const PLAYER: usize = 0;

pub struct EntityManager<'a> {
    pub entities: Vec<Entity<'a>>,
}

impl<'a> EntityManager<'a> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    // Push new entity to the entities vector
    pub fn create(&mut self, entity: Entity<'a>) {
        self.entities.push(entity);
    }

    pub fn update(&mut self, e: &mut sdl2::EventPump, input: &mut Input, counter: i32) {
        for entity in 0..self.entities.len() {
            self.entities[entity].update(counter);

            match entity {
                // Player should always be the 0th entity
                PLAYER => {
                    self.entities[PLAYER].input(e, input);
                }
                _ => {
                    self.entities[entity].position.x += 5;
                }
            }
        }
    }
}

impl EntityDefault for EntityManager<'_> {
    // Update and render every entity
    // added to the vector automatically
    fn update(&mut self) {}

    fn render(&self, canvas: &mut sdl2::render::WindowCanvas) -> Result<(), String> {
        for i in 0..self.entities.len() {
            self.entities[i].render(canvas)?;
        }
        Ok(())
    }
}
