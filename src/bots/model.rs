use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    hash,
    ops::{Add, AddAssign},
};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct WeightedMean {
    pub total_value: f64,
    pub total_weight: f64,
}

impl WeightedMean {
    pub fn new(value: f64, weight: f64) -> Self {
        Self {
            total_value: value,
            total_weight: weight,
        }
    }

    pub fn add_sample(&mut self, value: f64, weight: f64) {
        self.total_value += value * weight;
        self.total_weight += weight;
    }

    pub fn mean(&self) -> f64 {
        self.total_value / self.total_weight
    }
}

impl Add for WeightedMean {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.total_value += rhs.total_value;
        self.total_weight += rhs.total_weight;
        self
    }
}

impl AddAssign for WeightedMean {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct EvaluationEntry<P, V> {
    pub priority: P,
    pub value: V,
}

impl<P: std::cmp::PartialOrd, V: std::ops::AddAssign> EvaluationEntry<P, V> {
    pub fn update(&mut self, priority: P, value: V) {
        if self.priority == priority {
            self.value += value;
        } else if self.priority < priority {
            self.value = value;
            self.priority = priority;
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Model<K, P> {
    pub evaluation_memory: HashMap<K, EvaluationEntry<P, WeightedMean>>,
}

impl<K, P> Model<K, P> {
    pub fn new() -> Self {
        Self {
            evaluation_memory: HashMap::new(),
        }
    }
}

impl<K: hash::Hash + cmp::Eq, P: Default + Ord> Model<K, P> {
    pub fn evaluate(&self, key: &K) -> Option<f64> {
        self.evaluation_memory
            .get(key)
            .map(|entry| entry.value.mean())
    }

    pub fn learn(&mut self, key: K, value: f64, priority: P) {
        self.weighted_learn(key, value, 1.0, priority)
    }

    pub fn weighted_learn(&mut self, key: K, value: f64, weight: f64, priority: P) {
        self.weighted_learn_with_decay(key, value, weight, priority, 1.0)
    }

    pub fn weighted_learn_with_decay(
        &mut self,
        key: K,
        value: f64,
        weight: f64,
        priority: P,
        decay: f64,
    ) {
        let entry = self.evaluation_memory.entry(key).or_default();
        entry.value.total_value *= decay;
        entry.value.total_weight *= decay;

        match entry.priority.cmp(&priority) {
            Ordering::Greater => {}
            Ordering::Equal => entry.value.add_sample(value, weight),
            Ordering::Less => {
                entry.value = WeightedMean::new(value, weight);
                entry.priority = priority;
            }
        }
    }
}
