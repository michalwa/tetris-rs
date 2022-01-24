use game::{Game, GameEvent};
use piston_window::*;
use std::time::{Duration, Instant};

mod game;
mod grid;
mod math;
mod tetromino;

pub(crate) const GRID_SIZE: [usize; 2] = [10, 20];
pub(crate) const BLOCK_SIZE: f64 = 30.0;

fn main() {
    let mut game = Game::new(GRID_SIZE);

    let window_size = [
        GRID_SIZE[0] as f64 * BLOCK_SIZE,
        GRID_SIZE[1] as f64 * BLOCK_SIZE,
    ];

    let mut window: PistonWindow = WindowSettings::new("Tetris", window_size)
        .resizable(false)
        .build()
        .unwrap();

    let mut tick_interval = 300;
    let mut last_tick = Instant::now();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _device| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            game.render(c, g);
        });

        if Instant::now() - last_tick > Duration::from_millis(tick_interval) {
            last_tick = Instant::now();
            game.handle_event(GameEvent::Tick);
        }

        match event {
            Event::Input(
                Input::Button(ButtonArgs {
                    button,
                    state: ButtonState::Press,
                    ..
                }),
                _,
            ) => match button {
                Button::Keyboard(Key::A) => game.handle_event(GameEvent::MoveLeft),
                Button::Keyboard(Key::D) => game.handle_event(GameEvent::MoveRight),
                Button::Keyboard(Key::W) => game.handle_event(GameEvent::Rotate),
                Button::Keyboard(Key::S) => tick_interval = 50,
                _ => {}
            }
            Event::Input(
                Input::Button(ButtonArgs {
                    button,
                    state: ButtonState::Release,
                    ..
                }),
                _,
            ) => match button {
                Button::Keyboard(Key::S) => tick_interval = 300,
                _ => {}
            }
            _ => {}
        }
    }
}
