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

use super::model::weighted_avg::WeightedAvg;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SearchResult<A> {
    pub depth: u16,
    pub value: f32,
    pub action: A,
}

#[derive(Debug)]
pub enum SearchError {
    TimeOut,
    AtMaximumDepth,
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::TimeOut => write!(f, "reached deadline before finishing computation"),
            SearchError::AtMaximumDepth => write!(f, "we are already at maximum depth"),
        }
    }
}

pub struct Logger {
    pub cache_hit_chance_model: WeightedAvgModel<u16>,
    pub cache_hit_depth_model: WeightedAvgModel<u16>,
    pub deadline_miss_model: WeightedAvg,
    pub print_search_results: bool,
}

impl Logger {
    fn new() -> Self {
        Logger {
            cache_hit_chance_model: WeightedAvgModel::new(),
            cache_hit_depth_model: WeightedAvgModel::new(),
            deadline_miss_model: weighted_avg::WeightedAvg::new(),
            print_search_results: false,
        }
    }

    fn log_cache_hit(&mut self, depth: u16, result: &SearchResult<Direction>) {
        self.cache_hit_chance_model.learn(depth, 1.0, ());
        self.cache_hit_depth_model
            .learn(depth, result.depth.into(), ());
    }

    fn log_cache_miss(&mut self, depth: u16) {
        self.cache_hit_chance_model.learn(depth, 0.0, ());
    }

    fn log_search_result(&self, result: &SearchResult<Direction>) {
        if self.print_search_results {
            // TODO implement logging
            println!("{result:.2?}");
        }
    }

    fn log_search_start<T>(&self, _board: &T, _constraint: SearchConstraint) {}

    fn log_search_end(&self, result: &SearchResult<Direction>) {
        println!("{result:.2?}");
    }
}

pub struct MeanMax<const ROWS: usize, const COLS: usize> {
    pub player_cache: lru::LruCache<Board<ROWS, COLS>, SearchResult<Direction>>,
    pub deadline: Option<Instant>,
    pub model: WeightedAvgModel<heuristic::PreprocessedBoard>,
    pub logger: Logger,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SearchConstraint {
    pub deadline: Option<Instant>,
    pub max_depth: usize,
}

impl Default for SearchConstraint {
    fn default() -> Self {
        Self {
            deadline: None,
            max_depth: usize::MAX,
        }
    }
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
            logger: Logger::new(),
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

    fn evaluate_for_player(
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
                self.logger.log_cache_hit(depth, result);
                return Ok(*result);
            }
        }

        self.logger.log_cache_miss(depth);

        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return Err(SearchError::TimeOut);
            }
        }

        let all_actions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for (action, reward, new_board) in all_actions.into_iter().filter_map(|action| {
            let mut new_board = *board;
            // TODO replace with the actual reward
            let reward = 1.0;

            new_board
                .swipe(action)
                .then_some((action, reward, new_board))
        }) {
            let mut weighted_avg = weighted_avg::WeightedAvg::<f32>::new();
            let mut min_search_depth = u16::MAX;
            for (board, weight) in new_board.spawns() {
                let evaluation = self.evaluate_for_player(&board, depth - 1)?;
                min_search_depth = min_search_depth.min(evaluation.depth);

                weighted_avg.add_sample(evaluation.value, weight.into());
            }

            let value = weighted_avg.mean() + reward;

            if best_result.value <= value {
                best_result = SearchResult {
                    depth: min_search_depth.saturating_add(1),
                    value,
                    action,
                };
            }
        }

        self.player_cache.put(*board, best_result);
        if depth > 2 {
            self.train_model(board, best_result.value, depth);
        }

        Ok(best_result)
    }

    pub fn search_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        constraint: SearchConstraint,
    ) -> SearchResult<Direction> {
        self.logger.log_search_start(board, constraint);

        let cached_result = self.player_cache.get(board).cloned();
        let mut prev_result = cached_result.unwrap_or(self.act_by_heuristic(board));

        self.logger.log_search_result(&prev_result);

        loop {
            match self.search_deeper(&prev_result, constraint, board) {
                Ok(result) => {
                    self.logger.log_search_result(&result);
                    prev_result = result;
                }
                Err(_err) => {
                    self.logger.log_search_end(&prev_result);
                    break prev_result;
                }
            }
        }
    }

    fn search_deeper(
        &mut self,
        prev_result: &SearchResult<Direction>,
        constraint: SearchConstraint,
        board: &Board<ROWS, COLS>,
    ) -> Result<SearchResult<Direction>, SearchError> {
        self.deadline = constraint
            .deadline
            // Bring back the deadline to account for roll-up time
            .map(|deadline| deadline - Duration::from_micros(3));

        let new_depth = prev_result.depth.checked_add(1);
        let new_depth = new_depth
            .filter(|&depth| (depth as usize) <= constraint.max_depth)
            .ok_or(SearchError::AtMaximumDepth)?;

        self.evaluate_for_player(board, new_depth)
    }

    pub fn act(
        &mut self,
        board: &Board<ROWS, COLS>,
        search_constraint: SearchConstraint,
    ) -> Direction {
        self.search_until(board, search_constraint).action
    }
}
