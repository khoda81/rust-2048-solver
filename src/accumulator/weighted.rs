use std::{
    fmt::{Display, Write},
    ops::{Add, AddAssign, Div, Mul, MulAssign},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Weighted<T = f64, W = T> {
    pub value: T,
    pub weight: W,
}

impl<T: num::traits::Zero, W: num::traits::Zero> Default for Weighted<T, W> {
    fn default() -> Self {
        Self {
            value: T::zero(),
            weight: W::zero(),
        }
    }
}

impl<T: Mul<W, Output = T>, W: Clone> Weighted<T, W> {
    pub fn new_weighted(value: T, weight: W) -> Self {
        Self {
            value: value * weight.clone(),
            weight,
        }
    }
}

impl<T, W> Weighted<T, W> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Weighted<U, W> {
        Weighted {
            value: f(self.value),
            weight: self.weight,
        }
    }
}

impl<T, W: num::traits::One> Weighted<T, W> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            weight: W::one(),
        }
    }
}

impl<T, W> Weighted<T, W> {
    pub fn scale<S>(&mut self, scale: S)
    where
        S: Clone,
        T: MulAssign<S>,
        W: MulAssign<S>,
    {
        self.value *= scale.clone();
        self.weight *= scale;
    }

    pub fn scaled<S>(self, scale: S) -> Self
    where
        S: Clone,
        T: Mul<S, Output = T>,
        W: Mul<S, Output = W>,
    {
        Weighted {
            value: self.value * scale.clone(),
            weight: self.weight * scale,
        }
    }

    pub fn weighted_average<R>(self) -> R
    where
        T: Div<W, Output = R>,
    {
        self.value / self.weight
    }
}

impl<T: AddAssign, W: AddAssign> Add for Weighted<T, W> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.value += rhs.value;
        self.weight += rhs.weight;
        self
    }
}

impl<T: AddAssign, W: AddAssign> AddAssign for Weighted<T, W> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        self.weight += rhs.weight;
    }
}

impl<T: Display + Div<W, Output = T> + Clone, W: Display + Clone> Display for Weighted<T, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.clone().weighted_average().fmt(f)?;
        f.write_str(" (")?;
        self.value.fmt(f)?;
        f.write_char('/')?;
        self.weight.fmt(f)?;
        f.write_char(')')?;

        Ok(())
    }
}

// TODO: Rename current Weighted to Fraction and add a new type called weighted with proper semantics
