use std::fmt;

use crate::board::{Board, Direction};

pub struct Game<const ROWS: usize, const COLS: usize> {
    pub board: Board<ROWS, COLS>,
}

impl<const ROWS: usize, const COLS: usize> Game<ROWS, COLS> {
    pub fn create() -> Self {
        Game {
            board: Board::new().random_spawn(),
        }
    }

    pub fn step(&mut self, direction: Direction) -> bool {
        if self.board.swipe(direction) {
            self.board = self.board.random_spawn();
        }

        !self.board.is_lost()
    }
}

impl<const ROWS: usize, const COLS: usize> fmt::Debug for Game<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
