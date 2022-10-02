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
    pub fn new() -> Self {
        Game {
            board: Board::new().random_spawn(),
        }
    }

    pub fn step(&mut self, direction: Direction) -> bool {
        let new_board = self.board.move_(direction);

        if !new_board.rows_iter().eq(self.board.rows_iter()) {
            self.board = new_board.random_spawn();
        }

        self.board.lost()
    }
}

impl<const ROWS: usize, const COLS: usize> Debug for Game<ROWS, COLS> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.board)
        
    }
}

pub struct Solver<const ROWS: usize, const COLS: usize> {
    values: HashMap<Board<ROWS, COLS>, (u8, f64, Direction)>,
}

impl<const ROWS: usize, const COLS: usize> Solver<ROWS, COLS> {
    pub fn new() -> Self {
        Solver {
            values: HashMap::new(),
        }
    }

    pub fn heuristic(board: &Board<ROWS, COLS>) -> f64 {
        if board.lost() {
            return 0.;
        }

        let empty_cells = board.rows_iter().flatten().filter(|&x| x == 0).count();

        empty_cells as f64 + 1.
    }

    pub fn get(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u8,
        deadline: Instant,
    ) -> (u8, f64, Direction) {
        match self.values.get(board) {
            Some(&(cache_depth, expected, direction))
                if cache_depth >= depth || deadline <= Instant::now() =>
            {
                (cache_depth, expected, direction)
            }
            _ => {
                if depth == 0 || deadline <= Instant::now() {
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
                        let (_, expected, _) = self.get(&new_board, depth - 1, deadline);
                        numerator += weight * expected;
                        denominator += weight;
                    }

                    (denominator != 0.).then(|| (numerator / denominator, direction))
                })
                .fold((f64::NEG_INFINITY, None), |acc, (expected, direction)| {
                    if expected > acc.0 {
                        (expected, Some(direction))
                    } else {
                        acc
                    }
                });

                let result = match direction {
                    Some(direction) => (depth, expected, direction),
                    None => (depth, Self::heuristic(board), rand::random()),
                };

                self.values.insert(board.clone(), result);
                result
            }
        }
    }

    pub fn get_timed(
        &mut self,
        board: &Board<ROWS, COLS>,
        deadline: Instant,
    ) -> (u8, f64, Direction) {
        let mut depth = 0;

        loop {
            let (search_depth, expected, direction) = self.get(board, depth, deadline);
            println!(
                "depth: {}, expected: {:.2}, direction: {:?}",
                search_depth, expected, direction
            );

            if deadline <= Instant::now() {
                break (search_depth, expected, direction);
            }

            depth = search_depth + 1;
        }
    }
}
