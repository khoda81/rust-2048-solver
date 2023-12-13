use crate::{
    board::{Board, Direction},
    bots::{
        heuristic,
        model::{weighted::Weighted, AccumulationModel},
    },
    game::Transition,
};

use std::{
    fmt::Display,
    num::{NonZeroU8, NonZeroUsize},
    ops,
    time::{Duration, Instant},
};

use thiserror::Error;
pub mod logger;

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

    pub fn max_u8(self) -> u8 {
        self.bound().map(|bound| bound.get() - 1).unwrap_or(u8::MAX)
    }
}

impl Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bound::Bounded(_) => self.max_u8().fmt(f),
            Bound::Unlimited => '∞'.fmt(f),
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
    pub depth: u8,
    pub is_complete: bool,
}

impl Evaluation {
    const TERMINAL: Self = Evaluation {
        value: 0.0,
        depth: 0,
        is_complete: true,
    };

    pub fn depth_bound(&self) -> Bound {
        if self.is_complete {
            Bound::Unlimited
        } else {
            Bound::new(self.depth)
        }
    }
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

        if self.is_complete {
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
    pub logger: logger::Logger,
}

impl<const ROWS: usize, const COLS: usize> Default for MeanMax<ROWS, COLS> {
    fn default() -> Self {
        Self::new()
    }
}

type OptionEvaluation = Option<Evaluation>;
type EvaluationResult = Result<Evaluation, SearchError>;
type Decision = Option<EvaluatedAction<Action>>;
type DecisionResult = Result<Decision, SearchError>;

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
            logger: logger::Logger::new(),
        }
    }

    pub fn train_model(&mut self, state: &Board<ROWS, COLS>, eval: Evaluation) {
        let preprocessed_board = heuristic::preprocess_board(state);
        let prev_eval = self.model.entry(preprocessed_board).or_default();

        let decay = 0.995;
        prev_eval.total_value *= decay;
        prev_eval.total_weight *= decay;

        let weight = 2.0_f64.powi(eval.depth.into()) as heuristic::Eval;
        *prev_eval += Weighted::new_weighted(eval.value, weight);
    }

    pub fn heuristic(&self, state: &Board<ROWS, COLS>) -> heuristic::Eval {
        // Preprocess the board for the model
        let preprocessed = heuristic::preprocess_board(state);

        self.model
            .get(&preprocessed)
            .map(|&weighted| weighted.average_value())
            .unwrap_or_else(|| heuristic::heuristic(preprocessed))
    }

    fn evaluate_state(
        &mut self,
        state: &Board<ROWS, COLS>,
        depth_limit: Bound,
    ) -> EvaluationResult {
        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return Err(SearchError::TimeOut);
            }
        }

        let eval = match self.best_action(state, depth_limit)? {
            Some(eval_action) => eval_action.eval,
            None => Evaluation::TERMINAL,
        };

        Ok(eval)
    }

    fn best_action(&mut self, state: &Board<ROWS, COLS>, depth_limit: Bound) -> DecisionResult {
        let mut best_action: Decision = None;

        for transition in state.iter_transitions() {
            let eval = self.evaluate_transition(transition, depth_limit)?;

            match best_action {
                Some(prev_action) if prev_action.eval > eval => {}
                _ => {
                    let action = transition.action;
                    best_action = Some(EvaluatedAction { eval, action });
                }
            }
        }

        Ok(best_action)
    }

    fn evaluate_transition(
        &mut self,
        transition: Transition<Board<ROWS, COLS>, Direction>,
        depth_limit: Bound,
    ) -> EvaluationResult {
        let Some(eval_depth_limit) = depth_limit - 1 else {
            return Ok(Evaluation {
                value: self.heuristic(&transition.new_state) + transition.reward,
                depth: 0,
                is_complete: false,
            });
        };

        if let Some(eval) = self.cached_evaluation(&transition.new_state, depth_limit) {
            return Ok(eval);
        }

        let mut transition_value = Weighted::<heuristic::Eval>::default();
        let mut eval = Evaluation::TERMINAL;

        for (next_state, weight) in transition.new_state.spawns() {
            let next_eval = if next_state.is_lost() {
                Evaluation::TERMINAL
            } else {
                self.evaluate_state(&next_state, eval_depth_limit)?
            };

            (eval.is_complete, eval.depth) =
                (next_eval.is_complete, next_eval.depth).min((eval.is_complete, eval.depth));

            transition_value += Weighted::new_weighted(next_eval.value, weight.into());
        }

        eval.value = transition_value.average_value() + transition.reward;
        eval.depth = eval.depth.saturating_add(1);

        self.evaluation_cache.put(transition.new_state, eval);
        if eval.depth > 2 {
            self.train_model(&transition.new_state, eval);
        }

        Ok(eval)
    }

    fn cached_evaluation(
        &mut self,
        state: &Board<ROWS, COLS>,
        depth_limit: Bound,
    ) -> OptionEvaluation {
        let mut cache_result = self.evaluation_cache.get(state);

        if let Some(result) = cache_result {
            if !result.is_complete && result.depth < depth_limit.max_u8() {
                cache_result = None
            }
        };

        self.logger
            .register_lookup_result(cache_result, depth_limit);

        cache_result.copied()
    }

    pub fn search_until(
        &mut self,
        state: &Board<ROWS, COLS>,
        constraint: SearchConstraint,
    ) -> Decision {
        let search_id = self.logger.register_search_start(state, constraint);

        // Initial search depth
        let initial_depth_limit = match constraint.deadline {
            // If there is deadline start at depth 0 and go deeper
            Some(_) => Bound::new(0),
            // Else, search with the maximum depth
            None => constraint.max_depth,
        };

        // Remove the previous deadline for the initial search
        self.deadline = None;

        let mut decision: Decision = self
            .best_action(state, initial_depth_limit)
            .expect("searching with no constraint");

        self.deadline = constraint
            .deadline
            // Bring back the deadline to account for roll-up time
            .map(|deadline| deadline - Duration::from_micros(3));

        // Search deeper loop
        loop {
            self.logger.register_search_result(&search_id, &decision);

            let Some(last_result) = decision else { break };

            // Reached the max_depth, quit
            if constraint.max_depth <= last_result.eval.depth_bound() {
                break;
            }

            // Move the depth limit two levels higher
            let depth_limit = (last_result.eval.depth_bound() + 2)
                // Limit the depth by depth limit
                .min(constraint.max_depth);

            match self.best_action(state, depth_limit) {
                Ok(result) => decision = result,
                Err(_) => break,
            };
        }

        self.logger.register_search_end(search_id);
        decision
    }
}
