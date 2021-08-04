// Bascially just controller input for now
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            jump: false,
        }
    }
}
