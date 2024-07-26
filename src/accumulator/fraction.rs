use std::fmt::{Display, Write};
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Copy, Clone, Debug)]
struct Fraction<N, D> {
    pub numerator: N,
    pub denominator: D,
}

impl<N, D> Fraction<N, D> {
    pub fn evaluate(self) -> <N as Div<D>>::Output
    where
        N: Div<D>,
    {
        self.numerator / self.denominator
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

impl Add for Fraction<f32, f32> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl AddAssign for Fraction<f32, f32> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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
    pub fn new(value: T) -> Self
    where
        W: num::traits::One,
    {
        Self {
            value,
            weight: W::one(),
        }
    }

    pub fn new_weighted(value: T, weight: W) -> Self {
        Self { value, weight }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Weighted<U, W> {
        Weighted {
            value: f(self.value),
            weight: self.weight,
        }
    }
}

impl<T, W> Weighted<T, W> {
    pub fn evaluate(self) -> <T as Mul<W>>::Output
    where
        T: Mul<W>,
    {
        self.value * self.weight
    }
}

#[derive(Debug, Clone)]
pub struct WeightedAverage<N, D>(Fraction<N, D>);

impl<N, D> WeightedAverage<N, D> {
    pub fn evaluate(self) -> <N as Div<D>>::Output
    where
        N: Div<D>,
    {
        self.0.evaluate()
    }
}

impl<N: num::traits::Zero, D: num::traits::Zero> Default for WeightedAverage<N, D> {
    fn default() -> Self {
        Self(Fraction {
            numerator: N::zero(),
            denominator: D::zero(),
        })
    }
}

impl<N, D, T, W> Add<Weighted<T, W>> for WeightedAverage<N, D>
where
    N: Add<<T as Mul<W>>::Output, Output = N>,
    D: Add<W, Output = D>,
    T: Mul<W>,
    W: Clone,
{
    type Output = Self;

    fn add(self, rhs: Weighted<T, W>) -> Self::Output {
        Self(Fraction {
            denominator: self.0.denominator + rhs.weight.clone(),
            numerator: self.0.numerator + rhs.evaluate(),
        })
    }
}

impl<N, D, T, W> AddAssign<Weighted<T, W>> for WeightedAverage<N, D>
where
    N: AddAssign<<T as Mul<W>>::Output>,
    D: AddAssign<W>,
    T: Mul<W>,
    W: Clone,
{
    fn add_assign(&mut self, rhs: Weighted<T, W>) {
        self.0.denominator += rhs.weight.clone();
        self.0.numerator += rhs.evaluate();
    }
}

impl<N: Display + Div<D, Output = N> + Clone, D: Display + Clone> Display
    for WeightedAverage<N, D>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
