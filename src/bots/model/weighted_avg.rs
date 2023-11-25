use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct WeightedAvg {
    pub total_value: f64,
    pub total_weight: f64,
}

impl WeightedAvg {
    pub fn new() -> Self {
        Self::with_value(0.0, 0.0)
    }

    pub fn with_value(value: f64, weight: f64) -> Self {
        Self {
            total_value: value,
            total_weight: weight,
        }
    }

    pub fn add_sample(&mut self, value: f64, weight: f64) {
        self.total_value += value * weight;
        self.total_weight += weight;
    }

    pub fn scale(&mut self, scale: f64) {
        self.total_value *= scale;
        self.total_weight *= scale;
    }

    pub fn mean(&self) -> f64 {
        self.total_value / self.total_weight
    }
}

impl Add for WeightedAvg {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.total_value += rhs.total_value;
        self.total_weight += rhs.total_weight;
        self
    }
}

impl AddAssign for WeightedAvg {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
