use std::fmt::Debug;
use std::cmp::Ordering;

use crossterm::event::{Event, KeyCode, KeyEvent};
use rand::prelude::*;
use snake::Direction::{Down, Left, Right, Up};
use snake::{Direction, Game};

pub trait SnakeStrategy {
    fn decide_move<R: Rng + Debug>(&mut self, game: &Game<R>) -> Action;
}

pub enum Action {
    Move(Direction),
    GiveUp,
}

pub struct UserInput;
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

pub struct Rotate {}
impl SnakeStrategy for Rotate {
    fn decide_move<R: Rng + Debug>(&mut self, game: &Game<R>) -> Action {
        let (s_head_x, s_head_y) = game.snake_head();
        let (apple_x, apple_y) = game.apple_position();

        let (diff_apple_x, diff_apple_y) = (
            apple_x as isize - s_head_x as isize,
            apple_y as isize - s_head_y as isize,
        );
        let in_lower_half = s_head_y < (game.board().height() / 2);
        let in_upward_col = s_head_x % 2 == 0;
        let apple_in_same_half = (apple_y < (game.board().height() / 2)) == in_lower_half;

        let at_left = s_head_x == 0;
        let at_right = s_head_x == game.board().height() - 1;
        let at_bottom = s_head_y == 0;
        let at_top = s_head_y == game.board().height() - 1;

        let direction = match (at_left, at_right, at_bottom, at_top) {
            (true, _, _, true) => Right,
            (true, _, _, _) => Up,
            (_, true, true, _) => Left,
            (_, true, _, _) => Down,
            (false, false, true, _) => match (
                apple_in_same_half,
                in_upward_col,
                diff_apple_x,
                diff_apple_y == 0,
            ) {
                (true, true, 0 | -1, false) => Up,
                _ => Left,
            },
            (false, false, _, true) => match (
                apple_in_same_half,
                in_upward_col,
                diff_apple_x,
                diff_apple_y == 0,
            ) {
                (true, false, 0 | 1, false) => Down,
                _ => Right,
            },
            (false, false, false, false) => {
                #[allow(clippy::match_same_arms)] // Known false positive. Order matters
                match (
                    in_lower_half,
                    in_upward_col,
                    apple_in_same_half,
                    diff_apple_x,
                    diff_apple_y.cmp(&0),
                ) {
                    (true, true, _, -1, Ordering::Equal) => Left,
                    (true, true, true, -1 | 0, Ordering::Greater) => Up,
                    (true, true, _, _, _) => Left,
                    (true, false, _, _, _) => Down,

                    (false, false, _, 1, Ordering::Equal) => Right,
                    (false, false, true, 1 | 0, Ordering::Less) => Down,
                    (false, false, _, _, _) => Right,
                    (false, true, _, _, _) => Up,
                }
            }
        };

        if game.check_if_move_safe(direction).is_ok() {
            Action::Move(direction)
        } else if in_lower_half {
            Action::Move(Up)
        } else {
            Action::Move(Down)
        }
    }
}
