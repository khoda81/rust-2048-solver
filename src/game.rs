use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
    time::Instant,
};

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
        let new_board = self.board.move_(direction);

        if !new_board.rows_iter().eq(self.board.rows_iter()) {
            self.board = new_board.random_spawn();
        }

        self.board.is_lost()
    }
}

impl<const ROWS: usize, const COLS: usize> Debug for Game<ROWS, COLS> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.board)
    }
}

pub struct DFS<const ROWS: usize, const COLS: usize> {
    values: HashMap<Board<ROWS, COLS>, (u32, f64, Direction)>,
}

impl<const ROWS: usize, const COLS: usize> Default for DFS<ROWS, COLS> {
    fn default() -> Self {
        DFS {
            values: HashMap::default(),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> DFS<ROWS, COLS> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heuristic(board: &Board<ROWS, COLS>) -> f64 {
        board.rows_iter().flatten().filter(|&x| x == 0).count() as f64 + 1000.
    }

    pub fn evaluate_by_depth(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u32,
    ) -> (u32, f64, Direction) {
        if board.is_lost() {
            return (depth, 0., rand::random());
        }

        match self.values.get(board) {
            Some(&(cache_depth, expected, direction)) if depth <= cache_depth => {
                (cache_depth, expected, direction)
            }
            _ => {
                if depth == 0 {
                    return (0, Self::heuristic(board), rand::random());
                }

                let (expected, direction) = [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .into_iter()
                .filter_map(|direction| {
                    let new_board = board.move_(direction);

                    if new_board.rows_iter().eq(board.rows_iter()) {
                        return None;
                    }

                    let mut numerator = 0.;
                    let mut denominator = 0.;

                    for (new_board, weight) in new_board.spawns() {
                        let (_, expected, _) = self.evaluate_by_depth(&new_board, depth - 1);
                        numerator += weight * expected;
                        denominator += weight;
                    }

                    (denominator != 0.).then_some((numerator / denominator, direction))
                })
                .fold((f64::NEG_INFINITY, None), |acc, (expected, direction)| {
                    if expected > acc.0 {
                        (expected, Some(direction))
                    } else {
                        acc
                    }
                });

                let result = (depth, expected, direction.unwrap());

                self.values.insert(board.clone(), result);

                result
            }
        }
    }

    pub fn evaluate_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        deadline: Instant,
    ) -> (u32, f64, Direction) {
        let mut depth = 0;

        loop {
            let (search_depth, expected, direction) = self.evaluate_by_depth(board, depth);
            println!("depth: {search_depth}, expected: {expected:.2}, direction: {direction:?}");

            if deadline <= Instant::now() {
                return (search_depth, expected, direction);
            }

            depth = search_depth + 1;
        }
    }
}
