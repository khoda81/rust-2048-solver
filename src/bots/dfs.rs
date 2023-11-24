use crate::board::{Board, Direction};
use std::{
    fmt,
    time::{Duration, Instant},
};

use super::{heuristic, model::Model};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SearchResult<A> {
    pub depth: u16,
    pub value: f32,
    pub action: A,
}

#[derive(Debug)]
pub enum SearchError {
    TimeOut,
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeOut => write!(f, "reached deadline before finishing computation"),
        }
    }
}

pub struct DFS<const ROWS: usize, const COLS: usize> {
    pub player_cache: lru::LruCache<Board<ROWS, COLS>, SearchResult<Direction>>,
    pub deadline: Instant,
    pub model: Model<heuristic::PreprocessedBoard, u16>,
}

impl std::error::Error for SearchError {}

impl<const ROWS: usize, const COLS: usize> Default for DFS<ROWS, COLS> {
    fn default() -> Self {
        DFS {
            player_cache: lru::LruCache::new(1000000.try_into().unwrap()),
            deadline: Instant::now(),
            model: Model::new(),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> DFS<ROWS, COLS> {
    pub fn new() -> Self {
        Self::default()
    }

    fn preprocess_for_model(board: &Board<ROWS, COLS>) -> heuristic::PreprocessedBoard {
        (
            board.count_empty() as u32,
            *board.cells.iter().flatten().max().unwrap() as u32,
        )
    }

    pub fn train_model(&mut self, board: &Board<ROWS, COLS>, value: f32, depth: u16) {
        let value = value as f64;
        let weight = 2.0_f64.powi(depth.into());

        let priority = 0;
        let prerocessed_board = Self::preprocess_for_model(board);
        let decay = 0.999;

        self.model
            .weighted_learn_with_decay(prerocessed_board, value, weight, priority, decay)
    }

    pub fn heuristic(&self, board: &Board<ROWS, COLS>) -> f64 {
        // Preprocess the board for the model
        let preprocessed = Self::preprocess_for_model(board);

        self.model
            .evaluate(&preprocessed)
            .unwrap_or_else(|| heuristic::heuristic(preprocessed))
    }

    fn act_by_heuristic(&self, board: &Board<ROWS, COLS>) -> SearchResult<Direction> {
        SearchResult {
            depth: 0,
            value: self.heuristic(board) as f32,
            // action without any search
            action: Direction::Up,
        }
    }

    pub fn evaluate_for_player(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u16,
    ) -> Result<SearchResult<Direction>, SearchError> {
        let mut best_result = SearchResult {
            depth,
            value: 0.,
            // action on terminal states
            action: Direction::Up,
        };

        if board.is_lost() {
            best_result.depth = u16::MAX;
            return Ok(best_result);
        }

        if depth == 0 {
            return Ok(self.act_by_heuristic(board));
        }

        if let Some(result) = self.player_cache.get(board) {
            if result.depth >= depth {
                return Ok(*result);
            }
        }

        for action in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let mut new_board = board.clone();

            if !new_board.swipe(action) {
                continue;
            }

            // TODO replace with the actual reward
            let reward = 1.0;
            let value = self.evaluate_for_opponent(&new_board, depth - 1)? + reward;

            if best_result.value <= value {
                best_result.value = value;
                best_result.action = action;
            }
        }

        self.player_cache.put(board.clone(), best_result);
        self.train_model(board, best_result.value, depth);

        Ok(best_result)
    }

    #[inline(always)]
    pub fn evaluate_for_opponent(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u16,
    ) -> Result<f32, SearchError> {
        if Instant::now() >= self.deadline {
            return Err(SearchError::TimeOut);
        }

        let mut numerator = 0.;
        let mut denominator = 0.;
        let spawns = board.spawns();

        for (new_board, weight) in spawns {
            let evaluation = self.evaluate_for_player(&new_board, depth)?.value;

            numerator += weight as f32 * evaluation;
            denominator += weight as f32;
        }

        Ok(numerator / denominator)
    }

    pub fn evaluate_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        deadline: Instant,
    ) -> SearchResult<Direction> {
        // pessimistic deadline to end early instead of late
        self.deadline = deadline - Duration::from_micros(100);

        let mut result = self.act_by_heuristic(board);

        while let Some(new_result) = result
            .depth
            .checked_add(1)
            .and_then(|new_depth| self.evaluate_for_player(board, new_depth).ok())
        {
            result = new_result;
            // TODO implement logging
            println!("{result:.2?}");
        }

        result
    }

    pub fn act(&mut self, board: &Board<ROWS, COLS>, deadline: Instant) -> Direction {
        self.evaluate_until(board, deadline).action
    }
}
