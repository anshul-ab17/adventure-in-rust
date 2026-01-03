use graphics::*;
use opengl_graphics::GlGraphics;

use crate::core::game_object::{GameObject, Position};

pub struct Enemy {
    pub pos: Position,
    pub health: i32,
}

impl GameObject for Enemy {
    fn render(&self, c: &Context, gl: &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, 30.0);
        let transform = c.transform.trans(self.pos.x, self.pos.y);
        rectangle(color::GREEN, square, transform, gl);
    }

    fn update(&mut self, dt: f64) {
        self.pos.y += 40.0 * dt;
    }

    fn position(&self) -> Position {
        self.pos
    }

    fn radius(&self) -> f64 {
        15.0
    }
}
