use crate::{
    board::{Board, Cell, Direction},
    bots::{
        heuristic::{self, EmptyCount, MaxCell},
        model::{weighted::Weighted, AccumulationModel},
    },
    utils,
};

use std::{
    fmt,
    num::NonZeroUsize,
    time::{Duration, Instant},
};

const ALL_ACTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SearchResult<A> {
    pub depth: Depth,
    pub value: heuristic::Eval,
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
            SearchError::TimeOut => write!(f, "reached deadline before finishing search"),
            SearchError::AtMaximumDepth => write!(f, "already at maximum depth"),
        }
    }
}

pub type Depth = u8;
pub const MAX_DEPTH: Depth = Depth::MAX;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct SearchID(usize);
pub struct SearchInfo {
    pub constraint: SearchConstraint,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

pub struct Logger {
    pub cache_hit_chance_model: AccumulationModel<Depth, Weighted>,
    pub cache_hit_depth_model: AccumulationModel<Depth, Weighted>,
    pub deadline_miss_model: Weighted,
    pub search_log: Vec<SearchInfo>,
    pub print_search_results: bool,
    pub print_hit_info: bool,
}

impl Logger {
    fn new() -> Self {
        Logger {
            cache_hit_chance_model: AccumulationModel::new(),
            cache_hit_depth_model: AccumulationModel::new(),
            deadline_miss_model: Weighted::default(),
            search_log: Vec::new(),
            print_search_results: false,
            print_hit_info: false,
        }
    }

    fn register_cache_hit(&mut self, depth: Depth, result: &SearchResult<Direction>) {
        let hit = Weighted::new(1.0);
        self.cache_hit_chance_model.insert(depth, hit);
        let hit_depth = Weighted::new(result.depth.into());
        self.cache_hit_depth_model.insert(depth, hit_depth);
    }

    fn register_cache_miss(&mut self, depth: Depth) {
        let miss = Weighted::new(0.0);
        self.cache_hit_chance_model.insert(depth, miss);
    }

    fn register_search_result(
        &mut self,
        result: &SearchResult<Direction>,
        SearchID(_search_id): SearchID,
    ) {
        if self.print_search_results {
            println!("{result:.2?}");
        }
    }

    fn register_search_start<T>(&mut self, _board: &T, constraint: SearchConstraint) -> SearchID {
        let search_info = SearchInfo {
            constraint,
            start_time: Instant::now(),
            end_time: None,
        };

        self.search_log.push(search_info);
        SearchID(self.search_log.len() - 1)
    }

    fn register_search_end(
        &mut self,
        result: &SearchResult<Direction>,
        SearchID(search_id): SearchID,
    ) {
        let end_time = Instant::now();
        println!("{result:.2?}");

        let search_info = match self.search_log.get_mut(search_id) {
            Some(search_info) => search_info,
            None => return,
        };

        search_info.end_time = Some(end_time);

        if self.print_hit_info {
            println!("Hit chance per depth:");
            println!("{:.2}", self.cache_hit_chance_model);

            println!("Hit depth per depth:");
            println!("{:.2}", self.cache_hit_depth_model);
        }

        let deadline = match search_info.constraint.deadline {
            Some(deadline) => deadline,
            _ => return,
        };

        let miss_seconds = if deadline <= end_time {
            (end_time - deadline).as_secs_f64()
        } else {
            -(deadline - end_time).as_secs_f64()
        };

        let avg_miss_seconds = self.deadline_miss_model.weighted_average();
        let miss_err = (avg_miss_seconds - miss_seconds).abs();
        let outlier_threshold = Duration::from_millis(5);
        if miss_err.is_nan() || Duration::from_secs_f64(miss_err) <= outlier_threshold {
            self.deadline_miss_model += Weighted::new(miss_seconds);
        } else {
            eprintln!(
                "Ignoring miss since it has a high error ({miss_duration:?}>{outlier_threshold:?})",
                miss_duration = Duration::from_secs_f64(miss_err),
            );
        }

        let miss_duration = utils::get_signed_duration(miss_seconds);
        println!("Deadline missed by {miss_duration:?}",);

        let avg_miss_seconds = self.deadline_miss_model.weighted_average();
        let avg_miss = utils::get_signed_duration(avg_miss_seconds);
        println!("Avg miss: {avg_miss:?}");
    }
}

pub type Eval = Weighted<heuristic::Eval>;
pub struct MeanMax<const ROWS: usize, const COLS: usize> {
    pub player_cache: lru::LruCache<Board<ROWS, COLS>, SearchResult<Direction>>,
    pub deadline: Option<Instant>,
    pub model: AccumulationModel<heuristic::PreprocessedBoard, Eval>,
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
    const DEFAULT_CACHE_SIZE: usize = 10_000_000;

    pub fn new() -> Self {
        Self::new_with_cache_size(Self::DEFAULT_CACHE_SIZE.try_into().unwrap())
    }

    pub fn new_with_cache_size(capacity: NonZeroUsize) -> Self {
        Self {
            player_cache: lru::LruCache::new(capacity),
            deadline: None,
            model: AccumulationModel::new(),
            logger: Logger::new(),
        }
    }

    fn preprocess_for_model(board: &Board<ROWS, COLS>) -> heuristic::PreprocessedBoard {
        (
            EmptyCount(board.count_empty() as u8),
            MaxCell(*board.cells.iter().flatten().max().unwrap() as Cell),
        )
    }

    pub fn train_model(&mut self, board: &Board<ROWS, COLS>, value: heuristic::Eval, depth: Depth) {
        let preprocessed_board = Self::preprocess_for_model(board);
        let prev_eval = self.model.entry(preprocessed_board).or_default();

        let decay = 0.999;
        prev_eval.total_value *= decay;
        prev_eval.total_weight *= decay;

        let weight = 2.0_f64.powi(depth.into()) as heuristic::Eval;
        *prev_eval += Weighted::new_weighted(value, weight);
    }

    pub fn heuristic(&self, board: &Board<ROWS, COLS>) -> heuristic::Eval {
        // Preprocess the board for the model
        let preprocessed = Self::preprocess_for_model(board);

        self.model
            .get(&preprocessed)
            .map(|&eval| eval.weighted_average())
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

    fn evaluate_for_player(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: Depth,
    ) -> Result<SearchResult<Direction>, SearchError> {
        if board.is_lost() {
            return Ok(SearchResult {
                depth: MAX_DEPTH,
                value: 0.,
                // action on terminal states
                action: Direction::Up,
            });
        }

        if depth == 0 {
            return Ok(self.act_by_heuristic(board));
        }

        if let Some(result) = self.player_cache.get(board) {
            if result.depth >= depth {
                self.logger.register_cache_hit(depth, result);
                return Ok(*result);
            }
        }

        self.logger.register_cache_miss(depth);

        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return Err(SearchError::TimeOut);
            }
        }

        let transitions = ALL_ACTIONS.into_iter().filter_map(|action| {
            let mut new_board = *board;
            // TODO replace with the actual reward
            let reward = 1.0;

            new_board
                .swipe(action)
                .then_some((action, reward, new_board))
        });

        let mut best_result = SearchResult {
            depth,
            value: 0.,
            // action on terminal states
            action: Direction::Up,
        };

        for (action, reward, new_board) in transitions {
            let mut next_eval = Weighted::<heuristic::Eval>::default();
            let mut min_search_depth = MAX_DEPTH;
            for (board, weight) in new_board.spawns() {
                let evaluation = self.evaluate_for_player(&board, depth - 1)?;
                min_search_depth = min_search_depth.min(evaluation.depth);
                next_eval += Weighted::new_weighted(evaluation.value, weight.into());
            }

            let value = next_eval.weighted_average() + reward;

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
        let search_id = self.logger.register_search_start(board, constraint);

        let cached_result = self.player_cache.get(board).cloned();
        let mut prev_result = cached_result.unwrap_or(self.act_by_heuristic(board));

        self.logger.register_search_result(&prev_result, search_id);

        loop {
            match self.search_deeper(&prev_result, constraint, board) {
                Ok(result) => {
                    self.logger.register_search_result(&result, search_id);
                    prev_result = result;
                }
                Err(_err) => {
                    self.logger.register_search_end(&prev_result, search_id);
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
        let mut new_depth = new_depth
            .filter(|&depth| (depth as usize) <= constraint.max_depth)
            .ok_or(SearchError::AtMaximumDepth)?;

        if self.deadline.is_none() {
            let max_depth = constraint.max_depth.min(MAX_DEPTH as usize) as Depth;
            new_depth = new_depth.max(max_depth);
        }

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
