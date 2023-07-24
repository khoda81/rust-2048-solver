use crate::board::{Board, Direction};
use std::{
    collections::HashMap,
    fmt,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EvaluationEntry {
    pub depth: u32,
    pub value: f64,
}

pub struct DFS<const ROWS: usize, const COLS: usize> {
    evaluation_cache: HashMap<Board<ROWS, COLS>, EvaluationEntry>,
}
#[derive(Debug)]
pub struct TimeOut;

impl fmt::Display for TimeOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "reached deadline before finishing computation")
    }
}

impl std::error::Error for TimeOut {}

impl<const ROWS: usize, const COLS: usize> Default for DFS<ROWS, COLS> {
    fn default() -> Self {
        DFS {
            evaluation_cache: HashMap::with_capacity(2048),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> DFS<ROWS, COLS> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn heuristic(board: &Board<ROWS, COLS>) -> f64 {
        (board.count_empty() as f64 + 3.).powi(2)
    }

    fn evaluate_by_heuristic(board: &Board<ROWS, COLS>) -> EvaluationEntry {
        EvaluationEntry {
            depth: 0,
            value: Self::heuristic(board),
        }
    }

    pub fn evaluate_by_depth(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u32,
        deadline: Instant,
    ) -> EvaluationEntry {
        if depth == 0 {
            return Self::evaluate_by_heuristic(board);
        }

        if board.is_lost() {
            return EvaluationEntry {
                depth: u32::MAX,
                value: 0.,
            };
        }

        match self.evaluation_cache.get(board) {
            Some(entry) if entry.depth >= depth => *entry,
            _ => {
                let entry = self
                    .find_best_action(board, depth - 1, deadline)
                    .ok()
                    .map(|(value, _)| EvaluationEntry { depth, value })
                    .or_else(|| self.evaluation_cache.get(board).copied())
                    .unwrap_or(Self::evaluate_by_heuristic(board));

                self.evaluation_cache.insert(board.clone(), entry);

                entry
            }
        }
    }

    pub fn find_best_action(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u32,
        deadline: Instant,
    ) -> Result<(f64, Direction), TimeOut> {
        let mut best_action_value = (f64::NEG_INFINITY, Direction::Up);

        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let mut new_board = board.clone();
            let moved = new_board.swipe(direction);

            if !moved {
                continue;
            }

            let mut numerator = 0.;
            let mut denominator = 0.;

            for (new_board, weight) in new_board.spawns() {
                // TODO optimize
                if Instant::now() >= deadline {
                    return Err(TimeOut);
                }

                let evaluation = self.evaluate_by_depth(&new_board, depth, deadline).value;

                numerator += weight * evaluation;
                denominator += weight;
            }

            let value = numerator / denominator;
            if value > best_action_value.0 {
                best_action_value = (value, direction);
            }
        }

        Ok(best_action_value)
    }

    pub fn evaluate_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        deadline: Instant,
    ) -> EvaluationEntry {
        // pessimistic deadline to end early instead of late
        let deadline = deadline - Duration::from_micros(100);

        let mut evaluation = self.evaluate_by_depth(board, 0, deadline);

        while Instant::now() < deadline {
            evaluation = self.evaluate_by_depth(board, evaluation.depth + 1, deadline);
        }

        evaluation
    }

    pub fn act(&mut self, board: &Board<ROWS, COLS>, deadline: Instant) -> Direction {
        self.evaluate_until(board, deadline);
        let (_value, action) = self
            .find_best_action(board, 1, Instant::now() + Duration::from_millis(100))
            .unwrap();

        action
    }
}
