use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
    time::{Duration, Instant},
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EvaluationEntry {
    pub depth: u32,
    pub value: f64,
    pub action: Direction,
}

pub struct DFS<const ROWS: usize, const COLS: usize> {
    evaluation_cache: HashMap<Board<ROWS, COLS>, EvaluationEntry>,
}

impl<const ROWS: usize, const COLS: usize> Default for DFS<ROWS, COLS> {
    fn default() -> Self {
        DFS {
            evaluation_cache: HashMap::default(),
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
        board: &Board<ROWS, COLS>,
        depth: u32,
        deadline: Instant,
    ) -> Option<EvaluationEntry> {
        if board.is_lost() {
            Some(EvaluationEntry {
                depth: u32::MAX,
                value: 0.,
                action: Direction::Up,
            })
        } else if depth == 0 {
            Some(EvaluationEntry {
                depth: 0,
                value: Self::heuristic(board),
                action: rand::random(),
            })
        } else {
            Self::evaluate_with(board, deadline, |board| {
                Self::evaluate_by_depth(board, depth - 1, deadline).map(|entry| entry.value)
            })
            .map(|(value, action)| EvaluationEntry {
                depth,
                value,
                action,
            })
        }
    }

    pub fn cached_evaluate_by_depth(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u32,
        deadline: Instant,
    ) -> Option<EvaluationEntry> {
        if depth < 2 {
            return Self::evaluate_by_depth(board, depth, deadline);
        }

        if board.is_lost() {
            return Some(EvaluationEntry {
                depth: u32::MAX,
                value: 0.,
                action: Direction::Up,
            });
        }

        let entry = match self.evaluation_cache.get(board) {
            Some(entry) if depth <= entry.depth => *entry,
            _ => {
                let entry = Self::evaluate_with(board, deadline, |board| {
                    self.cached_evaluate_by_depth(board, depth - 1, deadline)
                        .map(|entry| entry.value)
                })
                .map(|(value, action)| EvaluationEntry {
                    depth,
                    value,
                    action,
                })?;

                self.evaluation_cache.insert(board.clone(), entry);

                entry
            }
        };

        Some(entry)
    }

    pub fn evaluate_with<F>(
        board: &Board<ROWS, COLS>,
        deadline: Instant,
        mut heuristic: F,
    ) -> Option<(f64, Direction)>
    where
        F: FnMut(&Board<ROWS, COLS>) -> Option<f64>,
    {
        [
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
                if Instant::now() >= deadline {
                    return None;
                }

                let evaluation = heuristic(&new_board)?;

                numerator += weight * evaluation;
                denominator += weight;
            }

            (denominator != 0.).then_some((numerator / denominator, direction))
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .or_else(|| {
            (Instant::now() <= deadline).then_some((f64::NEG_INFINITY, Direction::Up))
            // comment
        })
    }

    pub fn evaluate_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        deadline: Instant,
    ) -> EvaluationEntry {
        let deadline = deadline - Duration::from_micros(100);

        let mut depth = 0;
        let mut best_evaluation = EvaluationEntry {
            depth: 0,
            value: Self::heuristic(board),
            action: rand::random(),
        };

        while let Some(evaluation) = self.cached_evaluate_by_depth(board, depth, deadline) {
            best_evaluation = evaluation;
            depth = evaluation.depth + 1;
        }

        best_evaluation
    }
}
