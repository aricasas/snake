use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    /// List of snake part coords in order from head to tail
    snake: Vec<(usize, usize)>,
    /// Current coords of apple
    apple: (usize, usize),
    move_count: u32,
}
impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width >= 5);
        assert!(height >= 5);

        let snake = Self::initial_snake(width, height);
        let board = Board::new(width, height);

        Self {
            board,
            snake,
            apple: (0, 0),
            move_count: 0,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn snake(&self) -> &Vec<(usize, usize)> {
        &self.snake
    }
    pub fn apple(&self) -> (usize, usize) {
        self.apple
    }
    pub fn snake_len(&self) -> usize {
        self.snake.len()
    }
    pub fn snake_head(&self) -> (usize, usize) {
        self.snake[0]
    }
    pub fn move_snake(self, direction: Direction) -> Self {
        self.try_move_snake(direction).unwrap()
    }
    pub fn try_move_snake(self, direction: Direction) -> Result<Self, (Self, SnakeLost)> {
        // Move head
        // Shift body
        // Check head collisions

        todo!()
    }

    ///
    fn initial_snake(width: usize, height: usize) -> Vec<(usize, usize)> {
        todo!()
    }
    fn new_apple_pos(&self) -> Result<usize, NoSpaceLeft> {
        // Filter array for empty spaces
        let empty_spaces = self
            .board
            .vec()
            .iter()
            .enumerate()
            .filter(|(_, cell)| matches!(cell, Cell::Empty))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        if empty_spaces.is_empty(){
            
        }
        // If empty, no space left

        // Else, select random empty position

        todo!()
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
    pub fn new(width: usize, height: usize) -> Self {
        let board = vec![Cell::default(); width * height];
        Self {
            board,
            size: (width, height),
        }
    }
    pub fn size(&self) -> (usize, usize) {
        self.size
    }
    pub fn width(&self) -> usize {
        self.size.0
    }
    pub fn height(&self) -> usize {
        self.size.1
    }
    pub fn vec(&self) -> &Vec<Cell> {
        &self.board
    }
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

        // Temp var needed because the borrow checker complains if I use self.width() inside the index
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

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Snake,
    Apple,
}
impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
