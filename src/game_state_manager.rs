pub struct GameStateManager {
    pub start_screen: bool,
    pub level_one: bool,
    pub level_two: bool,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            start_screen: true,
            level_one: false,
            level_two: false,
        }
    }
}
