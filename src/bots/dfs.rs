use crate::{
    board::{Board, Direction},
    bots::{
        heuristic,
        model::{weighted::Weighted, AccumulationModel},
    },
    game::Transition,
    utils,
};

use std::{
    fmt::Display,
    num::NonZeroUsize,
    time::{Duration, Instant},
};
use thiserror::Error;

type Action = Direction;

pub type Depth = u8;
pub const MAX_DEPTH: Depth = Depth::MAX;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Evaluation {
    pub value: heuristic::Eval,
    pub depth: Depth,
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        write!(
            f,
            "{depth:3} -> {value:.*}",
            precision,
            value = self.value,
            depth = self.depth
        )
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SearchResult<A> {
    pub eval: Evaluation,
    pub action: A,
}

impl<A: Display> Display for SearchResult<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.eval.fmt(f)?;
        write!(f, ": {}", self.action)
    }
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("search time exceeded the deadline")]
    TimeOut,

    #[error("reached the maximum search depth")]
    AtMaximumDepth,

    #[error("attempting to search a lost position")]
    SearchingOnLostState,
}

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

    fn register_cache_hit(&mut self, depth: Depth, eval: &Evaluation) {
        if !self.print_hit_info {
            return;
        }

        let hit_chance = Weighted::new(1.0);
        self.cache_hit_chance_model.insert(depth, hit_chance);
        if eval.depth < MAX_DEPTH {
            let hit_depth = Weighted::new(eval.depth.into());
            self.cache_hit_depth_model.insert(depth, hit_depth);
        }
    }

    fn register_cache_miss(&mut self, depth: Depth) {
        if !self.print_hit_info {
            return;
        }

        let hit_chance = Weighted::new(0.0);
        self.cache_hit_chance_model.insert(depth, hit_chance);
    }

    fn register_search_start<T>(&mut self, _board: &T, constraint: SearchConstraint) -> SearchID {
        let start_time = Instant::now();
        let search_info = SearchInfo {
            constraint,
            start_time,
            end_time: None,
        };

        self.search_log.push(search_info);

        if self.print_search_results {
            println!();

            if let Some(deadline) = constraint.deadline {
                println!("Searching for {:?}", deadline.duration_since(start_time));
            }

            println!("Until depth {}", constraint.max_depth);
        }

        SearchID(self.search_log.len() - 1)
    }

    fn register_search_result(
        &mut self,
        result: &SearchResult<Action>,
        &SearchID(search_id): &SearchID,
    ) {
        if self.print_search_results {
            print!("{result:.2}");

            if let Some(search_info) = self.search_log.get_mut(search_id) {
                print!(" in {:?}", search_info.start_time.elapsed());
            }

            println!()
        }
    }

    fn register_search_end(&mut self, SearchID(search_id): SearchID) {
        let end_time = Instant::now();

        if self.print_search_results {
            println!();
        }

        let search_info = match self.search_log.get_mut(search_id) {
            Some(search_info) => search_info,
            None => return,
        };

        search_info.end_time = Some(end_time);

        if self.print_hit_info {
            println!("Hit chance per depth:");
            println!("{:.3}", self.cache_hit_chance_model);

            println!("Hit depth per depth:");
            println!("{:.4}", self.cache_hit_depth_model);
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

        let avg_miss_seconds = self.deadline_miss_model.average_value();
        let miss_err = (avg_miss_seconds - miss_seconds).abs();
        let outlier_threshold = Duration::from_millis(5);
        if miss_err.is_nan() || Duration::from_secs_f64(miss_err) <= outlier_threshold {
            self.deadline_miss_model += Weighted::new(miss_seconds);
        } else {
            eprintln!(
                "Ignoring miss since it has a high error ({miss_duration:.1?}>{outlier_threshold:.1?})",
                miss_duration = Duration::from_secs_f64(miss_err),
            );
        }

        let miss_duration = utils::get_signed_duration(miss_seconds);
        println!("Deadline missed by {miss_duration:?}",);

        let avg_miss_seconds = self.deadline_miss_model.average_value();
        let avg_miss = utils::get_signed_duration(avg_miss_seconds);
        println!("Avg miss: {avg_miss:?}");
    }
}

pub struct MeanMax<const ROWS: usize, const COLS: usize> {
    pub player_cache: lru::LruCache<Board<ROWS, COLS>, Evaluation>,
    pub deadline: Option<Instant>,
    pub model: AccumulationModel<heuristic::PreprocessedBoard, Weighted<heuristic::Eval>>,
    pub logger: Logger,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SearchConstraint {
    pub deadline: Option<Instant>,
    pub max_depth: Depth,
}

impl Default for SearchConstraint {
    fn default() -> Self {
        Self {
            deadline: None,
            max_depth: MAX_DEPTH,
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

    pub fn train_model(&mut self, board: &Board<ROWS, COLS>, eval: Evaluation) {
        let preprocessed_board = heuristic::preprocess_board(board);
        let prev_eval = self.model.entry(preprocessed_board).or_default();

        let decay = 0.995;
        prev_eval.total_value *= decay;
        prev_eval.total_weight *= decay;

        let weight = 2.0_f64.powi(eval.depth.into()) as heuristic::Eval;
        *prev_eval += Weighted::new_weighted(eval.value, weight);
    }

    pub fn heuristic(&self, board: &Board<ROWS, COLS>) -> heuristic::Eval {
        // Preprocess the board for the model
        let preprocessed = heuristic::preprocess_board(board);

        self.model
            .get(&preprocessed)
            .map(|&weighted| weighted.average_value())
            .unwrap_or_else(|| heuristic::heuristic(preprocessed))
    }

    fn evaluate_for_player(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth: Depth,
    ) -> Result<Evaluation, SearchError> {
        if depth == 0 {
            return Ok(Evaluation {
                depth: 0,
                value: self.heuristic(board),
            });
        }

        if let Some(eval) = self.player_cache.get(board) {
            if eval.depth >= depth {
                self.logger.register_cache_hit(depth, eval);
                return Ok(*eval);
            }
        }

        self.logger.register_cache_miss(depth);

        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return Err(SearchError::TimeOut);
            }
        }

        let mut eval = None;
        for transition in board.iter_transitions() {
            let new_eval = Some(self.evaluate_transition(transition, depth)?);

            if new_eval > eval {
                eval = new_eval;
            }
        }

        let eval = eval.ok_or(SearchError::SearchingOnLostState)?;

        self.player_cache.put(*board, eval);
        if depth > 2 {
            self.train_model(board, eval);
        }

        Ok(eval)
    }

    fn evaluate_transition(
        &mut self,
        transition: Transition<Board<ROWS, COLS>, Action>,
        depth: Depth,
    ) -> Result<Evaluation, SearchError> {
        let mut new_state_value = Weighted::<heuristic::Eval>::default();
        let mut min_search_depth = MAX_DEPTH;

        let next_state = transition.new_state.spawns();

        for (new_state, weight) in next_state {
            if new_state.is_lost() {
                new_state_value += Weighted::new(0.0);
                continue;
            }

            let Evaluation { value, depth } = self.evaluate_for_player(&new_state, depth - 1)?;
            min_search_depth = min_search_depth.min(depth);
            new_state_value += Weighted::new_weighted(value, weight.into());
        }

        let eval = Evaluation {
            depth: min_search_depth.saturating_add(1),
            value: new_state_value.average_value() + transition.reward,
        };

        Ok(eval)
    }

    pub fn search_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        constraint: SearchConstraint,
    ) -> SearchResult<Action> {
        let search_id = self.logger.register_search_start(board, constraint);

        let mut prev_result = None;

        loop {
            match self.search_deeper(board, prev_result.as_ref(), constraint) {
                Ok(result) => {
                    self.logger.register_search_result(&result, &search_id);
                    prev_result = Some(result);
                }
                Err(_) => {
                    self.logger.register_search_end(search_id);
                    break prev_result.unwrap();
                }
            }
        }
    }

    fn search_deeper(
        &mut self,
        board: &Board<ROWS, COLS>,
        prev_result: Option<&SearchResult<Action>>,
        constraint: SearchConstraint,
    ) -> Result<SearchResult<Action>, SearchError> {
        let prev_depth = prev_result
            .map(|prev_result| prev_result.eval.depth)
            .unwrap_or(0);

        self.deadline = constraint
            .deadline
            // Bring back the deadline to account for roll-up time
            .map(|deadline| deadline - Duration::from_micros(3));

        let max_depth = constraint.max_depth.min(MAX_DEPTH);
        let mut new_depth = prev_depth
            .checked_add(1)
            .filter(|&depth| depth <= max_depth)
            .ok_or(SearchError::AtMaximumDepth)?;

        if self.deadline.is_none() {
            new_depth = new_depth.max(max_depth);
        }

        let mut search_result: Option<SearchResult<Action>> = None;
        for transition in board.iter_transitions() {
            let eval = self.evaluate_transition(transition, new_depth)?;

            if matches!(search_result, Some(prev_search_result) if eval < prev_search_result.eval) {
                continue;
            }

            search_result = Some(SearchResult {
                eval,
                action: transition.action,
            })
        }

        search_result.ok_or(SearchError::SearchingOnLostState)
    }

    pub fn act(
        &mut self,
        board: &Board<ROWS, COLS>,
        search_constraint: SearchConstraint,
    ) -> Action {
        self.search_until(board, search_constraint).action
    }
}
