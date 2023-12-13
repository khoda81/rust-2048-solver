use std::fmt;

use crate::board::{Direction, StateOf2048};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transition<State, Action> {
    pub action: Action,
    pub reward: f64,
    pub next_state: State,
}

pub struct GameOf2048<const ROWS: usize, const COLS: usize> {
    pub state: StateOf2048<ROWS, COLS>,
}

impl<const ROWS: usize, const COLS: usize> GameOf2048<ROWS, COLS> {
    pub fn create() -> Self {
        GameOf2048 {
            state: StateOf2048::new().random_spawn(),
        }
    }

    pub fn step(&mut self, direction: Direction) -> bool {
        if self.state.swipe(direction) {
            self.state = self.state.random_spawn();
        }

        !self.state.is_lost()
    }
}

impl<const ROWS: usize, const COLS: usize> fmt::Debug for GameOf2048<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.state)
    }
}
