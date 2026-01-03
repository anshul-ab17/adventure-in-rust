use graphics::*;
use opengl_graphics::GlGraphics;

use crate::core::game_object::{GameObject, Position};

pub struct Player {
    pub pos: Position,
    pub size: f64,
}

impl GameObject for Player {
    fn render(&self, c: &Context, gl: &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, self.size);
        let transform = c.transform.trans(self.pos.x, self.pos.y);
        rectangle(color::RED, square, transform, gl);
    }

    fn update(&mut self, _dt: f64) {}

    fn position(&self) -> Position {
        self.pos
    }

    fn radius(&self) -> f64 {
        self.size / 2.0
    }
}
