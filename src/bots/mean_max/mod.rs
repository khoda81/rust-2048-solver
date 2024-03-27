pub mod logger;
pub mod max_depth;
pub mod mean_max_2048;

use crate::{
    bots::model::{weighted::Weighted, Accumulator},
    utils,
};
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
pub struct MeanMax<State, P> {
    pub deadline: Option<Instant>,
    pub depth_limit: max_depth::MaxDepth,
    pub evaluation_cache: lru::LruCache<State, Evaluation>,
    pub model: Accumulator<P, Weighted<f64>>,
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
            model: Accumulator::new(),
            logger: logger::Logger::new(),
        }
    }
}
