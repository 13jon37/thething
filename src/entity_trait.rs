use sdl2::render::WindowCanvas;

pub trait EntityDefault {
    fn update(&mut self);
    fn render(&self, canavs: &mut WindowCanvas) -> Result<(), String>;
}



