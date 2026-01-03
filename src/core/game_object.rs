use graphics::*;
use opengl_graphics::GlGraphics;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub trait GameObject {
    fn render(&self, c: &Context, gl: &mut GlGraphics);
    fn update(&mut self, dt: f64);
    fn position(&self) -> Position;
    fn radius(&self) -> f64;
}
