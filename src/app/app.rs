use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::input::{Key, RenderArgs};
use std::collections::HashSet;
use std::path::PathBuf;

use crate::{
    core::{
        collision::collides,
        game_object::{GameObject, Position},
    },
    entities::{
        player::Player,
        bullet::Bullet,
        enemy::Enemy,
    },
};

pub struct App {
    pub gl: GlGraphics,
    pub player: Player,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub score: i32,
    pub glyphs: GlyphCache<'static>,

    pub pressed_keys: HashSet<Key>,
    pub shoot_cooldown: f64,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        let font_path: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "assets",
            "fonts",
            "PxPlus_IBM_VGA_8x16.ttf",
        ]
        .iter()
        .collect();

        App {
            gl,
            player: Player {
                pos: Position { x: 460.0, y: 700.0 },
                size: 40.0,
            },
            bullets: vec![],
            enemies: vec![
                Enemy {
                    pos: Position { x: 200.0, y: 100.0 },
                    health: 1,
                },
                Enemy {
                    pos: Position { x: 500.0, y: 150.0 },
                    health: 1,
                },
                Enemy {
                    pos: Position { x: 350.0, y: 50.0 },
                    health: 1,
                },
            ],
            score: 0,
            glyphs: GlyphCache::new(font_path, (), TextureSettings::new())
                .expect("Font load failed"),

            pressed_keys: HashSet::new(),
            shoot_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let speed = 300.0;
 
        if self.pressed_keys.contains(&Key::Left) {
            self.player.pos.x -= speed * dt;
        }
        if self.pressed_keys.contains(&Key::Right) {
            self.player.pos.x += speed * dt;
        }
        if self.pressed_keys.contains(&Key::Up) {
            self.player.pos.y -= speed * dt;
        }
        if self.pressed_keys.contains(&Key::Down) {
            self.player.pos.y += speed * dt;
        }
 
        self.shoot_cooldown -= dt;
        if self.pressed_keys.contains(&Key::Space) && self.shoot_cooldown <= 0.0 {
            self.bullets.push(Bullet {
                pos: self.player.pos,
                speed: 500.0,
                ttl: 2.0,
            });
            self.shoot_cooldown = 0.15;
        }
 
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
 
        for enemy in &mut self.enemies {
            enemy.update(dt);
        }
 
        for bullet in &mut self.bullets {
            for enemy in &mut self.enemies {
                if collides(bullet, enemy) {
                    bullet.ttl = 0.0;
                    enemy.health = 0;
                    self.score += 10;
                }
            }
        }

        self.bullets.retain(|b| b.ttl > 0.0);
        self.enemies.retain(|e| e.health > 0);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let score = self.score;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::BLACK, gl);

            self.player.render(&c, gl);

            for enemy in &self.enemies {
                enemy.render(&c, gl);
            }

            for bullet in &self.bullets {
                bullet.render(&c, gl);
            }

            text::Text::new_color(color::WHITE, 16)
                .draw(
                    &format!("Score: {}", score),
                    &mut self.glyphs,
                    &DrawState::default(),
                    c.transform.trans(10.0, 20.0),
                    gl,
                )
                .unwrap();
        });
    }
}
