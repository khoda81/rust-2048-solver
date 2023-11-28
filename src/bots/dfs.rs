use crate::{
    board::{Board, Direction},
    bots::{
        heuristic,
        model::{weighted_avg, WeightedAvgModel},
    },
};

use std::{
    fmt,
    num::NonZeroUsize,
    time::{Duration, Instant},
};

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

pub struct MeanMax<const ROWS: usize, const COLS: usize> {
    pub player_cache: lru::LruCache<Board<ROWS, COLS>, SearchResult<Direction>>,
    pub deadline: Option<Instant>,
    pub model: WeightedAvgModel<heuristic::PreprocessedBoard>,
}

impl std::error::Error for SearchError {}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchConstraint {
    pub deadline: Option<Instant>,
    pub depth: Option<usize>,
}

impl<const ROWS: usize, const COLS: usize> Default for MeanMax<ROWS, COLS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const ROWS: usize, const COLS: usize> MeanMax<ROWS, COLS> {
    const DEFAULT_CACHE_SIZE: usize = 1_000_000;

    pub fn new() -> Self {
        Self::new_with_cache_size(Self::DEFAULT_CACHE_SIZE.try_into().unwrap())
    }

    pub fn new_with_cache_size(capacity: NonZeroUsize) -> Self {
        Self {
            player_cache: lru::LruCache::new(capacity),
            deadline: None,
            model: WeightedAvgModel::new(),
        }
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

        let prerocessed_board = Self::preprocess_for_model(board);
        let decay = 0.999;

        self.model
            .weighted_learn_with_decay(prerocessed_board, value, weight, (), decay)
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
        if let Some(_miss) = self
            .deadline
            .and_then(|deadline| Instant::now().checked_duration_since(deadline))
        {
            return Err(SearchError::TimeOut);
        }

        let mut weighted_avg = weighted_avg::WeightedAvg::new();

        for (new_board, weight) in board.spawns() {
            weighted_avg.add_sample(
                self.evaluate_for_player(&new_board, depth)?.value as f64,
                weight,
            );
        }

        Ok(weighted_avg.mean() as f32)
    }

    pub fn search_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        constraint: SearchConstraint,
    ) -> SearchResult<Direction> {
        // pessimistic deadline to end early instead of late
        self.deadline = constraint
            .deadline
            .map(|deadline| deadline - Duration::from_micros(273));

        let mut result = self.act_by_heuristic(board);
        let result = loop {
            let depth = match result.depth.checked_add(1) {
                Some(val) => val,
                None => break result,
            };

            if let Some(max_depth) = constraint.depth {
                if depth as usize > max_depth {
                    break result;
                }
            }

            result = match self.evaluate_for_player(board, depth) {
                Ok(result) => result,
                Err(_) => break result,
            }

            // println!("{result:.2?}");
        };

        // TODO implement logging
        println!("{result:.2?}");

        result
    }

    pub fn act(
        &mut self,
        board: &Board<ROWS, COLS>,
        search_constraint: SearchConstraint,
    ) -> Direction {
        self.search_until(board, search_constraint).action
    }
}
