use draw::to_coord;
use game::Game;
use piston_window::Button;
use piston_window::{clear, PressEvent, UpdateEvent};
use piston_window::{types::Color, PistonWindow, WindowSettings};

extern crate piston_window;
extern crate rand;
mod draw;
mod game;
mod snake;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord(width), to_coord(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |ctx, g, _| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&ctx, g);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
