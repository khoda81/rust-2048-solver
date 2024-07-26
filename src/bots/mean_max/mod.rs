pub mod logger;
pub mod max_depth;
pub mod mean_max_2048;

use crate::accumulator::fraction::{Weighted, WeightedAverage};
use crate::{bots::heuristic, game, utils};
use std::fmt::Debug;
use std::{cmp, fmt::Display, hash::Hash, time::Instant};
use thiserror::Error;

pub type Value = f32;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Evaluation {
    /// Expected value of the given state.
    pub value: Value,

    /// Minimum depth of searched tree ([max_depth::MaxDepth::Unlimited] means this is the eval of a full search tree).
    pub min_depth: max_depth::MaxDepth,
}

impl Evaluation {
    const TERMINAL: Self = Evaluation {
        value: 0.0,
        min_depth: max_depth::MaxDepth::Unlimited,
    };

    #[deprecated = "use `self.depth` instead"]
    pub fn fits_depth_bound(&self, bound: max_depth::MaxDepth) -> bool {
        self.min_depth >= bound
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.min_depth {
            max_depth::MaxDepth::Bounded(_) => write!(f, "{:2}", self.min_depth)?,
            max_depth::MaxDepth::Unlimited => write!(f, "complete")?,
        }

        let precision = f.precision().unwrap_or(2);
        write!(f, " -> {value:.*}", precision, value = self.value)?;

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
        write!(f, "{}: ", self.action)?;
        Display::fmt(&self.eval, f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Decision<A> {
    Act(EvaluatedAction<A>),
    Resign,
}

impl<A> Decision<A> {
    pub fn eval(&self) -> Evaluation {
        match self {
            Decision::Act(act) => act.eval,
            Decision::Resign => Evaluation::TERMINAL,
        }
    }

    fn max_by_eval(self, other: Self) -> Self {
        std::cmp::max_by(self, other, |a, b| {
            a.eval()
                .partial_cmp(&b.eval())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

impl<A: Display> Display for Decision<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decision::Act(act) => write!(f, "{act}"),
            Decision::Resign => write!(f, "resign"),
        }
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
    pub max_depth: max_depth::MaxDepth,
}

impl SearchConstraint {
    pub fn new() -> Self {
        Self {
            deadline: None,
            max_depth: max_depth::MaxDepth::Unlimited,
        }
    }

    pub fn check_deadline(&self) -> bool {
        match self.deadline {
            Some(deadline) => Instant::now() < deadline,
            None => true,
        }
    }

    pub fn has_lower_depth(&self) -> bool {
        (self.max_depth - 1).is_some()
    }

    #[must_use]
    pub fn with_deadline(mut self, deadline: Instant) -> Self {
        self.deadline = Some(deadline);
        self
    }

    #[must_use]
    pub fn with_max_depth(mut self, max_depth: max_depth::MaxDepth) -> Self {
        self.max_depth = max_depth;
        self
    }
}

impl Default for SearchConstraint {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for SearchConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_empty = true;
        if let Some(deadline) = self.deadline {
            let duration = deadline.duration_since(std::time::Instant::now());

            write!(f, "for {}", utils::HumanDuration(duration))?;
            is_empty = false;
        }

        match self.max_depth {
            max_depth::MaxDepth::Bounded(_) => {
                if !is_empty {
                    f.write_str(", ")?;
                }
                write!(f, "{} levels deep", self.max_depth)?;
            }
            max_depth::MaxDepth::Unlimited => {
                if is_empty {
                    write!(f, "for ever")?;
                }
            }
        };

        Ok(())
    }
}

// TODO: Add concurrency to cache and search
pub struct MeanMax<Game: game::GameState, Heuristic> {
    pub deadline: Option<Instant>,
    pub depth_limit: max_depth::MaxDepth,
    pub evaluation_cache: lru::LruCache<Game::Outcome, Evaluation>,
    pub heuristic: Heuristic,
    pub logger: logger::Logger,
}

// NOTE: This is equivalent to Decision
pub type OptionEvaluation = Option<Evaluation>;
pub type EvaluationResult = Result<Evaluation, SearchError>;
pub type DecisionResult<A> = Result<Decision<A>, SearchError>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Transition<G: game::GameState> {
    pub action: G::Action,
    pub reward: G::Reward,
    pub next: G,
}

impl<G, H> MeanMax<G, H>
where
    G: game::GameState,
    G::Outcome: Hash + cmp::Eq,
{
    fn cached_evaluation(&mut self, outcome: &G::Outcome) -> OptionEvaluation {
        let mut cached_eval = self.evaluation_cache.get(outcome).copied();

        if let Some(eval) = cached_eval.as_mut() {
            if eval.min_depth < self.depth_limit {
                cached_eval = None;
            }
        }

        self.logger
            .register_lookup_result(cached_eval.as_ref(), self.depth_limit);

        cached_eval
    }
}

impl<G, H> MeanMax<G, H>
where
    G: game::GameState + Clone + Display,
    G::Outcome: game::DiscreteDistribution<T = G> + Hash + cmp::Eq + Clone + Display,
    G::Action: game::Discrete + Clone + Display,
    Value: From<G::Reward> + From<<G::Outcome as game::DiscreteDistribution>::Weight>,
    H: heuristic::Heuristic<G::Outcome, Value>,
    <G::Outcome as game::DiscreteDistribution>::Weight: Debug,
{
    pub fn decide_until(&mut self, state: &G, constraint: SearchConstraint) -> Decision<G::Action> {
        let search_handle = self.logger.start_search(state, constraint);

        // Initial search depth
        self.depth_limit = match constraint.deadline {
            // If there is a deadline, start at depth 0 and go deeper
            Some(_) => max_depth::MaxDepth::new(0),
            // Otherwise, search with the maximum depth
            None => constraint.max_depth,
        };

        // Remove the previous deadline for the initial search
        self.deadline = None;

        let mut decision = self
            .make_decision(state)
            .expect("searching with no constraint");

        self.deadline = constraint.deadline;

        // Search deeper loop
        // PERF: this can be done concurrently
        loop {
            self.logger
                .register_search_result(&search_handle, &decision);

            // If last decision was Resign break
            let last_decision = match &decision {
                Decision::Act(last_decision) => last_decision,
                Decision::Resign => break,
            };

            // Reached the max_depth, abort
            if last_decision.eval.min_depth >= constraint.max_depth {
                break;
            }

            // Move the depth limit higher for a deeper search
            self.depth_limit = last_decision.eval.min_depth + 1;

            match self.make_decision(state) {
                Ok(new_decision) => decision = new_decision,
                Err(SearchError::TimeOut) => break,
            }
        }

        self.logger.end_search(search_handle);
        decision
    }

    pub fn evaluate_state(&mut self, state: &G) -> EvaluationResult {
        let in_the_past = |instant: Instant| !instant.elapsed().is_zero();

        if self.deadline.is_some_and(in_the_past) {
            Err(SearchError::TimeOut)
        } else {
            self.make_decision(state).map(|decision| decision.eval())
        }
    }

    pub fn make_decision(&mut self, state: &G) -> DecisionResult<<G as game::GameState>::Action> {
        let mut best_decision = Decision::Resign;

        for action in <G::Action as game::Discrete>::iter() {
            let (reward, outcome) = state.clone().outcome(action.clone());

            // TODO: Make this iterative instead of recursive.
            let eval = self.evaluate_outcome(outcome)?;
            let eval = Evaluation {
                value: eval.value + f32::from(reward),
                min_depth: eval.min_depth,
            };

            let new_decision = Decision::Act(EvaluatedAction { eval, action });
            best_decision = best_decision.max_by_eval(new_decision);
        }

        Ok(best_decision)
    }

    fn evaluate_outcome(&mut self, outcome: G::Outcome) -> EvaluationResult {
        if outcome.clone().into_iter().next().is_none() {
            return Ok(Evaluation::TERMINAL);
        }

        if let Some(evaluation) = self.cached_evaluation(&outcome) {
            return Ok(evaluation);
        }

        // Decrease depth limit for the recursive call
        self.depth_limit = match self.depth_limit - 1 {
            Some(depth_limit) => depth_limit,
            None => {
                let evaluation = Evaluation {
                    value: self.heuristic.eval(&outcome),
                    min_depth: max_depth::MaxDepth::new(0),
                };

                return Ok(evaluation);
            }
        };

        let mut mean_value = WeightedAverage::<Value, Value>::default();
        let mut min_depth = max_depth::MaxDepth::Unlimited;

        for weighted in outcome.clone() {
            let eval = self.evaluate_state(&weighted.value)?;

            min_depth = std::cmp::min(eval.min_depth, min_depth);
            mean_value += Weighted {
                value: eval.value,
                weight: f32::from(weighted.weight),
            };
        }

        let eval = Evaluation {
            value: mean_value.evaluate(),
            min_depth: min_depth + 1,
        };

        if eval.min_depth.max_u8() > 2 {
            self.heuristic.update(outcome.clone(), eval.value);
        }

        self.evaluation_cache.put(outcome, eval);

        self.depth_limit += 1;
        Ok(eval)
    }
}
