use std::{self, fmt::Display, num::NonZeroU8, ops};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaxDepth {
    /// Represents any of {`0`, `1`, `2`, ..., `254`}
    Bounded(NonZeroU8),

    /// Represents `∞`
    #[default]
    Unlimited,
}

impl MaxDepth {
    pub fn new(max_value: u8) -> Self {
        max_value
            .checked_add(1)
            .and_then(NonZeroU8::new)
            .map_or(Self::Unlimited, Self::Bounded)
    }

    const fn bound(self) -> Option<NonZeroU8> {
        match self {
            Self::Bounded(bound) => Some(bound),
            Self::Unlimited => None,
        }
    }

    pub fn max_u8(self) -> u8 {
        self.bound().map_or(u8::MAX, |bound| bound.get() - 1)
    }

    /// Returns `true` if the max depth is [`Unlimited`].
    ///
    /// [`Unlimited`]: MaxDepth::Unlimited
    #[must_use]
    pub fn is_unlimited(&self) -> bool {
        matches!(self, Self::Unlimited)
    }
}

impl Display for MaxDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaxDepth::Bounded(_) => self.max_u8().fmt(f),
            MaxDepth::Unlimited => '∞'.fmt(f),
        }
    }
}

impl ops::Add<u8> for MaxDepth {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self.bound()
            .and_then(|bound| bound.checked_add(rhs))
            .map_or(Self::Unlimited, Self::Bounded)
    }
}

impl ops::AddAssign<u8> for MaxDepth {
    fn add_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

impl ops::Sub<u8> for MaxDepth {
    type Output = Option<Self>;

    fn sub(self, rhs: u8) -> Self::Output {
        match self.bound() {
            Some(bound) => NonZeroU8::new(bound.get().saturating_sub(rhs)).map(Self::Bounded),
            None => Some(MaxDepth::Unlimited),
        }
    }
}

impl ops::SubAssign<u8> for MaxDepth {
    fn sub_assign(&mut self, rhs: u8) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod test_depth {
    use std::num::NonZeroU8;

    use crate::bots::mean_max::max_depth::MaxDepth;
    #[test]
    fn test_new() {
        assert_eq!(MaxDepth::new(0).bound(), NonZeroU8::new(1))
    }

    #[test]
    fn test_order() {
        assert!(MaxDepth::new(0) < MaxDepth::Unlimited);
        assert!(MaxDepth::new(254) < MaxDepth::Unlimited);
    }
}
