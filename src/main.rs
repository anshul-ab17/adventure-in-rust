mod app;
mod core;
mod entities;

use app::app::App;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

const WIDTH: u32 = 960;
const HEIGHT: u32 = 768;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Adventures in Rust", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.pressed_keys.insert(key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.pressed_keys.remove(&key);
        }

        if let Some(u) = e.update_args() {
            app.update(u.dt);
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
