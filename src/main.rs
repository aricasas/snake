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

mod strats;

use snake::{Game, SnakeLost};

use crate::strats::{Action, Rotate, SnakeStrategy};

fn main() {
    let mut game = Game::new(6, 6);
    let mut strat = Rotate {};

    let final_message = loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
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
