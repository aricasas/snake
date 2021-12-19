#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use std::fmt::Debug;

use crossterm::event::{Event, KeyCode, KeyEvent};
use rand::prelude::*;
use snake::{Direction, Game, SnakeLost};

fn main() {
    let mut game = Game::new(10, 10);
    let mut strat = UserInput;

    let final_message = loop {
        game.print();

        game = match strat.decide_move(&game) {
            Action::Move(direction) => match game.try_move_snake(direction) {
                Ok((game, win)) => {
                    if win {
                        game.print();
                        break "You won!";
                    }
                    game
                }
                Err((_, reason)) => match reason {
                    SnakeLost::RanIntoWall => break "You ran into a wall dummy",
                    SnakeLost::RanIntoSnake => break "You ran into yourself dummy",
                },
            },
            Action::GiveUp => break "You gave up like the loser you are",
        };
    };

    println!("Game over!\n{}", final_message);
}

struct UserInput;
impl SnakeStrategy for UserInput {
    fn decide_move<R: Rng + Debug>(&mut self, _game: &Game<R>) -> Action {
        crossterm::terminal::enable_raw_mode().unwrap();

        let action = loop {
            match crossterm::event::read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: _,
                }) => break Action::Move(Direction::Up),
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: _,
                }) => break Action::Move(Direction::Down),
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: _,
                }) => break Action::Move(Direction::Right),
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: _,
                }) => break Action::Move(Direction::Left),
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: _,
                }) => break Action::GiveUp,

                _ => {}
            }
        };

        crossterm::terminal::disable_raw_mode().unwrap();

        action
    }
}

trait SnakeStrategy {
    fn decide_move<R: Rng + Debug>(&mut self, game: &Game<R>) -> Action;
}

enum Action {
    Move(Direction),
    GiveUp,
}
