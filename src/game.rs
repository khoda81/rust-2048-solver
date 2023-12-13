use std::fmt;

use crate::board::{Direction, StateOf2048};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transition<State, Action> {
    pub action: Action,
    pub reward: f64,
    pub next_state: State,
}

pub struct GameOf2048<const ROWS: usize, const COLS: usize> {
    pub board: StateOf2048<ROWS, COLS>,
}

impl<const ROWS: usize, const COLS: usize> GameOf2048<ROWS, COLS> {
    pub fn create() -> Self {
        GameOf2048 {
            board: StateOf2048::new().random_spawn(),
        }
    }

    pub fn step(&mut self, direction: Direction) -> bool {
        if self.board.swipe(direction) {
            self.board = self.board.random_spawn();
        }

        !self.board.is_lost()
    }
}

impl<const ROWS: usize, const COLS: usize> fmt::Debug for GameOf2048<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
