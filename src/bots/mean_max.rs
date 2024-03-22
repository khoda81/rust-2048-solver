pub mod logger;
pub mod max_depth;
pub mod mean_max_2048;

use crate::bots::model::{weighted::Weighted, AccumulationModel};
use std::{fmt::Display, hash::Hash, num::NonZeroUsize, time::Instant};
use thiserror::Error;

type Value = f32;

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
        write!(f, " -> {value:.*}", precision, value = self.value,)?;

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
    pub max_depth: max_depth::MaxDepth,
}

impl Default for SearchConstraint {
    fn default() -> Self {
        Self {
            deadline: None,
            max_depth: max_depth::MaxDepth::Unlimited,
        }
    }
}

impl SearchConstraint {
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

// TODO: Add concurrency to cache and search
pub struct MeanMax<State, P> {
    pub deadline: Option<Instant>,
    pub depth_limit: max_depth::MaxDepth,
    pub evaluation_cache: lru::LruCache<State, Evaluation>,
    pub model: AccumulationModel<P, Weighted<f64>>,
    pub logger: logger::Logger,
}

impl<S: Hash + Eq, P> Default for MeanMax<S, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Hash + Eq, P> MeanMax<S, P> {
    const DEFAULT_CACHE_SIZE: usize = 10_000_000;

    pub fn new() -> Self {
        Self::new_with_cache_size(Self::DEFAULT_CACHE_SIZE.try_into().unwrap())
    }

    pub fn new_with_cache_size(capacity: NonZeroUsize) -> Self {
        Self {
            evaluation_cache: lru::LruCache::new(capacity),
            deadline: None,
            depth_limit: max_depth::MaxDepth::Unlimited,
            model: AccumulationModel::new(),
            logger: logger::Logger::new(),
        }
    }
}
