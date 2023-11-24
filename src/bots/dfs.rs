use crate::board::{Board, Direction};
use std::{
    fmt,
    time::{Duration, Instant},
};

use super::{heuristic, model::Model};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SearchResult<A> {
    pub depth: u32,
    pub value: f64,
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
    pub model: Model<heuristic::PreprocessedBoard>,
}

impl std::error::Error for SearchError {}

impl<const ROWS: usize, const COLS: usize> Default for DFS<ROWS, COLS> {
    fn default() -> Self {
        DFS {
            player_cache: lru::LruCache::new(10000000.try_into().unwrap()),
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

    pub fn add_heuristic_sample(&mut self, board: &Board<ROWS, COLS>, priority: u32, eval: f64) {
        self.model
            .learn(Self::preprocess_for_model(board), eval, priority)
    }

    pub fn heuristic(&self, board: &Board<ROWS, COLS>) -> f64 {
        // Preprocess the board for the model
        let preprocessed = Self::preprocess_for_model(board);

        self.model
            .evaluate(&preprocessed)
            .map(|eval| eval.get_value())
            .unwrap_or_else(|| heuristic::heuristic(preprocessed))
    }

    fn act_by_heuristic(&self, board: &Board<ROWS, COLS>) -> SearchResult<Direction> {
        SearchResult {
            depth: 0,
            value: self.heuristic(board),
            // action without any search
            action: Direction::Up,
        }
    }

    pub fn evaluate_for_player(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u32,
    ) -> Result<SearchResult<Direction>, SearchError> {
        let mut best_result = SearchResult {
            depth,
            value: 0.,
            // action on terminal states
            action: Direction::Up,
        };

        if board.is_lost() {
            best_result.depth = u32::MAX;
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
        self.add_heuristic_sample(
            board,
            depth,
            // 0,
            best_result.value,
        );

        Ok(best_result)
    }

    #[inline(always)]
    pub fn evaluate_for_opponent(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: u32,
    ) -> Result<f64, SearchError> {
        if Instant::now() >= self.deadline {
            return Err(SearchError::TimeOut);
        }

        let mut numerator = 0.;
        let mut denominator = 0.;
        let spawns = board.spawns();

        for (new_board, weight) in spawns {
            let evaluation = self.evaluate_for_player(&new_board, depth)?.value;

            numerator += weight * evaluation;
            denominator += weight;
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

        while let Ok(new_result) = self.evaluate_for_player(board, result.depth + 1) {
            println!("{new_result:.2?}");
            result = new_result;

            if result.depth == u32::MAX {
                break;
            }
        }

        result
    }

    pub fn act(&mut self, board: &Board<ROWS, COLS>, deadline: Instant) -> Direction {
        self.evaluate_until(board, deadline).action
    }
}
