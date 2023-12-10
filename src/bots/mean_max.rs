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
    fmt::{Display, Write},
    num::{NonZeroU8, NonZeroUsize},
    ops,
    time::{Duration, Instant},
};

use thiserror::Error;

type Action = Direction;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bound {
    /// Represents any of {`0`, `1`, `2`, ..., `254`}
    Bounded(NonZeroU8),

    /// Represents `∞`
    #[default]
    Unlimited,
}

impl Bound {
    pub fn new(max_value: u8) -> Self {
        max_value
            .checked_add(1)
            .and_then(NonZeroU8::new)
            .map(Self::Bounded)
            .unwrap_or(Self::Unlimited)
    }

    fn bound(self) -> Option<NonZeroU8> {
        match self {
            Self::Bounded(bound) => Some(bound),
            Self::Unlimited => None,
        }
    }

    pub fn bound_u8(self) -> u8 {
        self.bound().map(|bound| bound.get() - 1).unwrap_or(u8::MAX)
    }
}

impl Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bound::Bounded(n) => n.fmt(f),
            Bound::Unlimited => f.write_char('∞'),
        }
    }
}

impl ops::Add<u8> for Bound {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self.bound()
            .and_then(|bound| bound.checked_add(rhs))
            .map(Self::Bounded)
            .unwrap_or(Self::Unlimited)
    }
}

impl ops::AddAssign<u8> for Bound {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl ops::Sub<u8> for Bound {
    type Output = Option<Self>;

    fn sub(self, rhs: u8) -> Self::Output {
        match self.bound() {
            Some(bound) => NonZeroU8::new(bound.get().saturating_sub(rhs)).map(Self::Bounded),
            None => Some(Bound::Unlimited),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Evaluation {
    pub value: heuristic::Eval,
    pub is_terminal: bool,
    pub depth: u8,
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
        )?;

        if self.is_terminal {
            write!(f, " terminal")?;
        }

        Ok(())
    }
}

#[must_use]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct EvaluatedAction<A> {
    pub eval: Evaluation,
    pub action: A,
}

impl<A: Display> Display for EvaluatedAction<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} :", self.action)?;
        self.eval.fmt(f)
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
    pub cache_hit_chance_model: AccumulationModel<Bound, Weighted>,
    pub cache_hit_depth_model: AccumulationModel<Bound, Weighted>,
    pub deadline_miss_model: Weighted,
    pub search_log: Vec<SearchInfo>,

    // Config
    pub log_search_results: bool,
    pub log_hit_info: bool,
    pub clear_screen: bool,
}

impl Logger {
    fn new() -> Self {
        Logger {
            cache_hit_chance_model: AccumulationModel::new(),
            cache_hit_depth_model: AccumulationModel::new(),
            deadline_miss_model: Weighted::default(),
            search_log: Vec::new(),

            log_search_results: false,
            log_hit_info: false,
            clear_screen: false,
        }
    }

    fn register_cache_hit(&mut self, depth: Bound, eval: &Evaluation) {
        if !self.log_hit_info {
            return;
        }

        let hit = Weighted::new(1.0);
        self.cache_hit_chance_model.add_to(depth, hit);

        let hit_depth = Weighted::new(eval.depth.into());
        self.cache_hit_depth_model.add_to(depth, hit_depth);
    }

    fn register_cache_miss(&mut self, depth: Bound) {
        if !self.log_hit_info {
            return;
        }

        let miss = Weighted::new(0.0);
        self.cache_hit_chance_model.add_to(depth, miss);
    }

    fn register_lookup_result(&mut self, result: Option<&Evaluation>, depth_limit: Bound) {
        match result {
            Some(result) => self.register_cache_hit(depth_limit, result),
            None => self.register_cache_miss(depth_limit),
        }
    }

    fn register_search_start<T>(&mut self, _board: &T, constraint: SearchConstraint) -> SearchID {
        let start_time = Instant::now();
        let search_info = SearchInfo {
            constraint,
            start_time,
            end_time: None,
        };

        self.search_log.push(search_info);

        if self.log_search_results {
            println!();

            if let Some(deadline) = constraint.deadline {
                println!("Searching for {:?}", deadline.duration_since(start_time));
            }

            if let Some(max_depth) = constraint.max_depth.bound() {
                println!("Until depth {}", max_depth);
            }
        }

        SearchID(self.search_log.len() - 1)
    }

    fn register_search_result(
        &mut self,
        &SearchID(search_id): &SearchID,
        result: &EvaluatedAction<Action>,
    ) {
        if self.log_search_results {
            print!("{result:.2}");

            if let Some(search_info) = self.search_log.get_mut(search_id) {
                print!(" in {:?}", search_info.start_time.elapsed());
            }

            println!()
        }
    }

    fn register_search_end(&mut self, SearchID(search_id): SearchID) {
        let end_time = Instant::now();

        if self.log_search_results {
            println!();
        }

        if self.clear_screen {
            println!("\x1b[2J\x1b[H");
        }

        let search_info = match self.search_log.get_mut(search_id) {
            Some(search_info) => search_info,
            None => return,
        };

        search_info.end_time = Some(end_time);

        if self.log_hit_info {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SearchConstraint {
    pub deadline: Option<Instant>,
    pub max_depth: Bound,
}

impl Default for SearchConstraint {
    fn default() -> Self {
        Self {
            deadline: None,
            max_depth: Bound::Unlimited,
        }
    }
}

pub struct MeanMax<const ROWS: usize, const COLS: usize> {
    pub evaluation_cache: lru::LruCache<Board<ROWS, COLS>, Evaluation>,
    pub deadline: Option<Instant>,
    pub model: AccumulationModel<heuristic::PreprocessedBoard, Weighted<heuristic::Eval>>,
    pub logger: Logger,
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
            evaluation_cache: lru::LruCache::new(capacity),
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

    fn evaluate_state(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth_limit: Bound,
    ) -> Result<Evaluation, SearchError> {
        let Some(lower_depth) = depth_limit - 1 else {
            return Ok(Evaluation {
                is_terminal: false,
                depth: 0,
                value: self.heuristic(board),
            });
        };

        // println!("Searching at Bound: {depth_limit}");

        if let Some(eval) = self.lookup_in_cache(board, depth_limit) {
            return Ok(eval);
        }

        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return Err(SearchError::TimeOut);
            }
        }

        let mut eval = self.best_action(board, lower_depth)?.eval;

        self.evaluation_cache.put(*board, eval);
        eval.depth = eval.depth.saturating_add(1);
        if eval.depth > 2 {
            self.train_model(board, eval);
        }

        Ok(eval)
    }

    fn best_action(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth_limit: Bound,
    ) -> Result<EvaluatedAction<Action>, SearchError> {
        let mut best_action: Result<EvaluatedAction<_>, _> = Err(SearchError::SearchingOnLostState);

        for transition in board.iter_transitions() {
            let eval = self.evaluate_transition(transition, depth_limit)?;

            match best_action {
                Ok(prev_action) if prev_action.eval > eval => {}
                _ => {
                    best_action = Ok(EvaluatedAction {
                        eval,
                        action: transition.action,
                    });
                }
            }
        }

        best_action
    }

    fn evaluate_transition(
        &mut self,
        transition: Transition<Board<ROWS, COLS>, Direction>,
        depth_limit: Bound,
    ) -> Result<Evaluation, SearchError> {
        let mut new_state_value = Weighted::<heuristic::Eval>::default();
        let mut eval = Evaluation {
            is_terminal: true,
            depth: u8::MAX,
            value: 0.0,
        };

        let next_states = transition.new_state.spawns();
        for (next_state, weight) in next_states {
            if next_state.is_lost() {
                new_state_value += Weighted::new_weighted(0.0, weight.into());
                continue;
            }

            let Evaluation {
                value,
                depth,
                is_terminal,
            } = self.evaluate_state(&next_state, depth_limit)?;

            eval.is_terminal &= is_terminal;
            eval.depth = eval.depth.min(depth);
            new_state_value += Weighted::new_weighted(value, weight.into());
        }

        eval.value = new_state_value.average_value() + transition.reward;

        Ok(eval)
    }

    fn lookup_in_cache(
        &mut self,
        board: &Board<ROWS, COLS>,
        depth_limit: Bound,
    ) -> Option<Evaluation> {
        let mut cache_result = self.evaluation_cache.get(board);

        match cache_result {
            Some(result) if result.is_terminal => {}
            Some(result) if result.depth < depth_limit.bound_u8() => cache_result = None,

            _ => {}
        };

        self.logger
            .register_lookup_result(cache_result, depth_limit);

        cache_result.copied()
    }

    pub fn search_until(
        &mut self,
        board: &Board<ROWS, COLS>,
        constraint: SearchConstraint,
    ) -> EvaluatedAction<Action> {
        let search_id = self.logger.register_search_start(board, constraint);

        // Initial search depth
        let initial_depth_limit = match constraint.deadline {
            // If there is deadline start at depth 0 and go deeper
            Some(_) => Bound::new(0),
            // Else, search with the maximum depth
            None => constraint.max_depth,
        };

        // Remove the previous deadline for the initial search
        self.deadline = None;

        let mut result = self
            .best_action(board, initial_depth_limit)
            .expect("searching with no constraint");

        self.deadline = constraint
            .deadline
            // Bring back the deadline to account for roll-up time
            .map(|deadline| deadline - Duration::from_micros(3));

        // Search deeper loop
        loop {
            self.logger.register_search_result(&search_id, &result);

            let current_depth = if result.eval.is_terminal {
                Bound::Unlimited
            } else {
                Bound::new(result.eval.depth)
            };

            // Reached the max_depth, quit
            if constraint.max_depth <= current_depth {
                break;
            }

            // Move the depth limit two levels higher
            let depth_limit = (current_depth + 2)
                // Limit the depth by depth limit
                .min(constraint.max_depth);

            result = match self.best_action(board, depth_limit) {
                Ok(result) => result,
                Err(_) => break,
            };
        }

        self.logger.register_search_end(search_id);
        result
    }
}
