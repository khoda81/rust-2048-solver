use std::{
    fmt::{Display, Write},
    ops::{Add, AddAssign, Div, Mul, MulAssign},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Weighted<T = f64, W = T> {
    pub total_value: T,
    pub total_weight: W,
}

impl<T: num::traits::Zero, W: num::traits::Zero> Default for Weighted<T, W> {
    fn default() -> Self {
        Self {
            total_value: T::zero(),
            total_weight: W::zero(),
        }
    }
}

impl<T: Mul<W, Output = T>, W: Clone> Weighted<T, W> {
    pub fn new_weighted(value: T, weight: W) -> Self {
        Self {
            total_value: value * weight.clone(),
            total_weight: weight,
        }
    }
}

impl<T, W: num::traits::One> Weighted<T, W> {
    pub fn new(value: T) -> Self {
        Self {
            total_value: value,
            total_weight: W::one(),
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
        self.total_value *= scale.clone();
        self.total_weight *= scale;
    }

    pub fn scaled<S>(self, scale: S) -> Self
    where
        S: Clone,
        T: Mul<S, Output = T>,
        W: Mul<S, Output = W>,
    {
        Weighted {
            total_value: self.total_value * scale.clone(),
            total_weight: self.total_weight * scale,
        }
    }

    pub fn weighted_average<R>(self) -> R
    where
        T: Div<W, Output = R>,
    {
        self.total_value / self.total_weight
    }
}

impl<T: AddAssign, W: AddAssign> Add for Weighted<T, W> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.total_value += rhs.total_value;
        self.total_weight += rhs.total_weight;
        self
    }
}

impl<T: AddAssign, W: AddAssign> AddAssign for Weighted<T, W> {
    fn add_assign(&mut self, rhs: Self) {
        self.total_value += rhs.total_value;
        self.total_weight += rhs.total_weight;
    }
}

impl<T: Display + Div<W, Output = T> + Clone, W: Display + Clone> Display for Weighted<T, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.clone().weighted_average().fmt(f)?;
        f.write_str(" (")?;
        self.total_value.fmt(f)?;
        f.write_char('/')?;
        self.total_weight.fmt(f)?;
        f.write_char(')')?;

        Ok(())
    }
}
