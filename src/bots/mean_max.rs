pub mod logger;
pub mod mean_max_2048;

use crate::bots::model::{weighted::Weighted, AccumulationModel};
use std::{
    fmt::Display,
    hash::Hash,
    num::{NonZeroU8, NonZeroUsize},
    ops,
    time::Instant,
};
use thiserror::Error;

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
            .map_or(Self::Unlimited, Self::Bounded)
    }

    fn bound(self) -> Option<NonZeroU8> {
        match self {
            Self::Bounded(bound) => Some(bound),
            Self::Unlimited => None,
        }
    }

    pub fn max_u8(self) -> u8 {
        self.bound().map_or(u8::MAX, |bound| bound.get() - 1)
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
            .map_or(Self::Unlimited, Self::Bounded)
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

type Value = f64;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Evaluation {
    pub value: Value,
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

    pub fn fits_depth_bound(&self, bound: Bound) -> bool {
        self.is_complete || self.depth >= bound.max_u8()
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
            write!(f, " complete")?;
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
}

pub struct MeanMax<State, P> {
    pub deadline: Option<Instant>,
    pub depth_limit: Bound,
    pub evaluation_cache: lru::LruCache<State, Evaluation>,
    pub model: AccumulationModel<P, Weighted<Value>>,
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
            depth_limit: Bound::Unlimited,
            model: AccumulationModel::new(),
            logger: logger::Logger::new(),
        }
    }
}
