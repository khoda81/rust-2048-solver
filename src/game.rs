use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.board)
    }
}

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
        write!(f, "Reached deadline before finishing computation")
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
        board.rows_iter().flatten().filter(|&x| x == 0).count() as f64 + 1000.
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
                    .map(|(value, _)| EvaluationEntry { depth, value })
                    .ok()
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
            let new_board = board.move_(direction);

            if new_board.rows_iter().eq(board.rows_iter()) {
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
