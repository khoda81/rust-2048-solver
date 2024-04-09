use std::fmt::{Display, Write};
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Fraction<N, D> {
    pub numerator: N,
    pub denominator: D,
}

impl<N, D> Fraction<N, D> {
    pub fn evaluate<R>(self) -> R
    where
        N: Div<D, Output = R>,
    {
        self.numerator / self.denominator
    }
}

impl<N, D: num::traits::One> Fraction<N, D> {
    pub fn new(numerator: N) -> Self {
        Self {
            numerator,
            denominator: D::one(),
        }
    }
}

impl<N: num::traits::Zero, D: num::traits::Zero> Default for Fraction<N, D> {
    fn default() -> Self {
        Self {
            numerator: N::zero(),
            denominator: D::zero(),
        }
    }
}

impl<N: AddAssign, D: AddAssign> Add for Fraction<N, D> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.numerator += rhs.numerator;
        self.denominator += rhs.denominator;
        self
    }
}

impl<N: AddAssign, D: AddAssign> AddAssign for Fraction<N, D> {
    fn add_assign(&mut self, rhs: Self) {
        self.numerator += rhs.numerator;
        self.denominator += rhs.denominator;
    }
}

impl<N: Display + Div<D, Output = N> + Clone, D: Display + Clone> Display for Fraction<N, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.clone().evaluate().fmt(f)?;
        f.write_str(" (")?;
        self.numerator.fmt(f)?;
        f.write_char('/')?;
        self.denominator.fmt(f)?;
        f.write_char(')')?;

        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Weighted<T, W> {
    pub value: T,
    pub weight: W,
}

impl<T, W> Weighted<T, W> {
    pub fn new(value: T, weight: W) -> Self {
        Self { value, weight }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Weighted<U, W> {
        Weighted {
            value: f(self.value),
            weight: self.weight,
        }
    }
}

impl<T, W: Clone> Weighted<T, W> {
    pub fn to_fraction<N>(self) -> Fraction<N, W>
    where
        T: Mul<W, Output = N>,
    {
        Fraction {
            numerator: self.value * self.weight.clone(),
            denominator: self.weight,
        }
    }
}
