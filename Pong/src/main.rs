extern crate piston_window;

mod ball;
mod game;
mod paddle;

use piston_window::types::Color;
use piston_window::*;

use game::Game;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BACKGROUND: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Pong", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game: Game = Game::new([WIDTH as f64, HEIGHT as f64]);
    let mut held: bool = false;
    let mut key_pressed: char = 'n';

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            held = true;
            match key {
                Key::Up => {
                    key_pressed = 'u';
                }
                Key::Down => {
                    key_pressed = 'd';
                }
                _ => {}
            }
        }
        if let Some(Button::Keyboard(_key)) = event.release_args() {
            held = false;
            key_pressed = 'n';
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear(BACKGROUND, graphics);
            game.update_game(&context, graphics, key_pressed, held);
        });
    }
}

