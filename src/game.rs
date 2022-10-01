use std::fmt;

use crate::board::{Board, Direction};

pub struct Game<const ROWS: usize, const COLS: usize> {
    board: Board<ROWS, COLS>,
}

impl<const ROWS: usize, const COLS: usize> Game<ROWS, COLS> {
    pub fn new() -> Self {
        Game {
            board: Board::new().random_spawn(),
        }
    }

    pub fn step(&mut self, direction: Direction) -> bool {
        let new_board = match direction {
            Direction::Left => self.board.move_left(),
            Direction::Right => self.board.move_right(),
            Direction::Up => self.board.move_up(),
            Direction::Down => self.board.move_down(),
        };

        if !new_board.rows_iter().eq(self.board.rows_iter()) {
            self.board = new_board.random_spawn();
        }

        self.board.lost()
    }
}

impl<const ROWS: usize, const COLS: usize> fmt::Debug for Game<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
