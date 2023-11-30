use std::{
    fmt::{Display, Write},
    ops::{Add, AddAssign, Div, Mul, MulAssign},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WeightedAvg<T = f64, W = T> {
    pub total_value: T,
    pub total_weight: W,
}

impl<T: num::traits::Zero> Default for WeightedAvg<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: num::traits::Zero> WeightedAvg<T> {
    pub fn new() -> Self {
        Self {
            total_value: T::zero(),
            total_weight: T::zero(),
        }
    }
}

impl<T> WeightedAvg<T> {
    pub fn with_value(value: T, weight: T) -> Self {
        Self {
            total_value: value,
            total_weight: weight,
        }
    }
}

impl<T> WeightedAvg<T>
where
    T: Mul<Output = T> + AddAssign<<T as Mul>::Output> + Clone,
{
    pub fn add_sample(&mut self, value: T, weight: T) {
        self.total_value += value * weight.clone();
        self.total_weight += weight;
    }
}

impl<T: MulAssign + Clone> WeightedAvg<T> {
    pub fn scale(&mut self, scale: T) {
        self.total_value *= scale.clone();
        self.total_weight *= scale;
    }
}

impl<T: Clone, W: Clone> WeightedAvg<T, W> {
    pub fn mean<R>(&self) -> R
    where
        T: Div<W, Output = R>,
    {
        self.total_value.clone() / self.total_weight.clone()
    }
}

impl<T: AddAssign> Add for WeightedAvg<T> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.total_value += rhs.total_value;
        self.total_weight += rhs.total_weight;
        self
    }
}

impl<T, Rhs> AddAssign<Rhs> for WeightedAvg<T>
where
    Self: Clone,
    WeightedAvg<T>: Add<Rhs, Output = WeightedAvg<T>>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        *self = self.clone() + rhs;
    }
}

impl<T: Display + Div<Output = T> + Clone> Display for WeightedAvg<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.mean().fmt(f)?;
        f.write_str(" (")?;
        self.total_value.fmt(f)?;
        f.write_char('/')?;
        self.total_weight.fmt(f)?;
        f.write_char(')')?;

        Ok(())
    }
}
