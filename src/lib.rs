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

use std::fmt::{Debug, Display};
use std::ops::{Index, IndexMut};

use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Game<R: Rng + Debug> {
    board: Board,
    /// List of snake part coords in order from head to tail
    snake: Vec<(usize, usize)>,
    apple_position: (usize, usize),
    move_count: u32,
    rng: R,
}
impl Game<ThreadRng> {
    #[must_use]
    /// # Panics
    ///
    /// Will panic if board smaller than 5x5
    pub fn new(width: usize, height: usize) -> Self {
        let rng = rand::thread_rng();

        Self::new_with_rng(width, height, rng)
    }
}
impl<R: Rng + Debug> Game<R> {
    /// # Panics
    ///
    /// Will panic if board smaller than 5x5
    pub fn new_with_rng(width: usize, height: usize, mut rng: R) -> Self {
        assert!(width >= 5);
        assert!(height >= 5);

        let mut board = Board::new(width, height);

        // Add snake
        let snake = Self::initial_snake((width, height));
        snake
            .iter()
            .for_each(|&position| board[position] = Cell::Snake);

        // Add apple
        let apple_position = Self::new_apple_pos(&board, &mut rng).unwrap();
        board[apple_position] = Cell::Apple;

        Self {
            board,
            snake,
            apple_position,
            move_count: 0,
            rng,
        }
    }

    /// # Panics
    ///
    /// Will panic if the move loses the game
    pub fn move_snake(self, direction: Direction) -> (Self, bool) {
        self.try_move_snake(direction).unwrap()
    }
    /// Tries to move the snake into the direction passed in.
    /// Returns an `Ok` value with the modified `Game` if the move was successful, also sends a `true` value when the last apple is eaten.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the move loses the game
    /// Returns an `Err` with the unchanged `Game` and the `SnakeLost` enum with the reason for losing
    #[allow(clippy::missing_panics_doc)] // The panic from unwrapping is impossible
    pub fn try_move_snake(self, direction: Direction) -> Result<(Self, bool), (Self, SnakeLost)> {
        // Move head
        let old_head = self.snake[0];

        let new_head = match direction {
            Direction::Up => (old_head.0, {
                if old_head.1 + 1 == self.board.height() {
                    return Err((self, SnakeLost::RanIntoWall));
                }
                old_head.1 + 1
            }),
            Direction::Down => (old_head.0, {
                if old_head.1 == 0 {
                    return Err((self, SnakeLost::RanIntoWall));
                }
                old_head.1 - 1
            }),
            Direction::Right => (
                {
                    if old_head.0 + 1 == self.board.width() {
                        return Err((self, SnakeLost::RanIntoWall));
                    }
                    old_head.0 + 1
                },
                old_head.1,
            ),
            Direction::Left => (
                {
                    if old_head.0 == 0 {
                        return Err((self, SnakeLost::RanIntoWall));
                    }
                    old_head.0 - 1
                },
                old_head.1,
            ),
        };

        // It's okay to unwrap because snake is never empty
        let &old_tail = self.snake.last().unwrap();

        // Check head collisions
        if let Cell::Snake = self.board[new_head] {
            if new_head != old_tail {
                return Err((self, SnakeLost::RanIntoSnake));
            }
        }

        // Create mutable binding once it's okay to modify because no errors will appear anymore
        let mut this = self;

        // Shift body
        this.snake.rotate_right(1);
        this.snake[0] = new_head;

        // Move snake in board
        let before_head = this.board[new_head];
        this.board[new_head] = Cell::Snake;

        let mut game_won = false;

        if let Cell::Apple = before_head {
            // Re-add old tail to snake
            this.snake.push(old_tail);
            // Add new apple
            match Self::new_apple_pos(&this.board, &mut this.rng) {
                Ok(apple_pos) => {
                    this.board[apple_pos] = Cell::Apple;
                }
                Err(_) => game_won = true,
            }
        } else {
            // Delete old tail from board
            this.board[old_tail] = Cell::Empty;
        }

        this.move_count += 1;
        Ok((this, game_won))
    }

    fn initial_snake(board_size: (usize, usize)) -> Vec<(usize, usize)> {
        let center_x = ((board_size.0 + 1) / 2) - 1;
        let center_y = ((board_size.1 + 1) / 2) - 1;

        // Center tile and two below
        // We now the two tiles below the center exist because
        // Game constructor checks board size is at least 5x5
        vec![
            (center_x, center_y),
            (center_x, center_y - 1),
            (center_x, center_y - 2),
        ]
    }
    fn new_apple_pos(board: &Board, rng: &mut R) -> Result<(usize, usize), NoSpaceLeft> {
        // Filter array for empty spaces
        let empty_spaces = board
            .vec()
            .iter()
            .enumerate()
            .filter(|(_, cell)| matches!(cell, Cell::Empty))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        // If empty, no space left to put in apple
        // Else, select random empty position
        let apple_pos = match empty_spaces.len() {
            0 => Err(NoSpaceLeft),
            _ => Ok(*empty_spaces.choose(rng).unwrap()),
        }?;

        Ok((apple_pos % board.width(), apple_pos / board.width()))
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn snake(&self) -> &Vec<(usize, usize)> {
        &self.snake
    }
    pub fn apple_position(&self) -> (usize, usize) {
        self.apple_position
    }
    pub fn snake_len(&self) -> usize {
        self.snake.len()
    }
    pub fn snake_head(&self) -> (usize, usize) {
        self.snake[0]
    }
    pub fn print(&self) {
        print!(
            "Move Count: {}  Apple position: {:?}\n{}",
            self.move_count, self.apple_position, self.board
        );
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SnakeLost {
    RanIntoWall,
    RanIntoSnake,
}
#[derive(Debug, Clone, Copy)]
struct NoSpaceLeft;

#[derive(Debug, Clone)]
pub struct Board {
    /// Vec with the cells that form the board
    board: Vec<Cell>,
    /// Width and height of board
    size: (usize, usize),
}
impl Board {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let board = vec![Cell::default(); width * height];
        Self {
            board,
            size: (width, height),
        }
    }
    #[must_use]
    pub const fn size(&self) -> (usize, usize) {
        self.size
    }
    #[must_use]
    pub const fn width(&self) -> usize {
        self.size.0
    }
    #[must_use]
    pub const fn height(&self) -> usize {
        self.size.1
    }
    #[must_use]
    pub const fn vec(&self) -> &Vec<Cell> {
        &self.board
    }
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Can't be const because it runs destructor on self
    pub fn into_inner(self) -> Vec<Cell> {
        self.board
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.width());
        assert!(index.1 < self.height());

        &self.board[index.0 + self.width() * index.1]
    }
}
impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.width());
        assert!(index.1 < self.height());

        // Temp var needed because the borrow checker complains if I use self.width() index self
        let w = self.width();

        &mut self.board[index.0 + w * index.1]
    }
}
impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}
impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[index]
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board
            .chunks_exact(self.width()) // Divide into rows
            .rev() // Print from top to bottom
            .try_for_each(|chunk| {
                writeln!(f).and_then(|_| chunk.iter().try_for_each(|cell| write!(f, "{} ", cell)))
            })
            .and_then(|_| writeln!(f))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Snake,
    Apple,
}
impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "\u{2591}"),
            Cell::Snake => write!(f, "s"),
            Cell::Apple => write!(f, "a"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
