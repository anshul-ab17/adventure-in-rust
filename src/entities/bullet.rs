use graphics::*;
use opengl_graphics::GlGraphics;

use crate::core::game_object::{GameObject, Position};

pub struct Bullet {
    pub pos: Position,
    pub speed: f64,
    pub ttl: f64,
}

impl GameObject for Bullet {
    fn render(&self, c: &Context, gl: &mut GlGraphics) {
        let square = rectangle::square(0.0, 0.0, 6.0);
        let transform = c.transform.trans(self.pos.x, self.pos.y);
        rectangle(color::WHITE, square, transform, gl);
    }

    fn update(&mut self, dt: f64) {
        self.pos.y -= self.speed * dt;
        self.ttl -= dt;
    }

    fn position(&self) -> Position {
        self.pos
    }

    fn radius(&self) -> f64 {
        3.0
    }
}
