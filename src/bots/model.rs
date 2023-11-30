// TODO rename to models

use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    fmt::{Display, Write},
    hash,
};

use itertools::Itertools;

pub mod preprocessor;
pub mod weighted_avg;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct EvaluationEntry<P, V> {
    pub priority: P,
    pub value: V,
}

// TODO: make this generic over the numeric type of weighted avg
#[derive(Clone, Debug, Default)]
pub struct WeightedAvgModel<I, P = ()> {
    pub memory: HashMap<I, EvaluationEntry<P, weighted_avg::WeightedAvg>>,
}

impl<X, P> WeightedAvgModel<X, P> {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }
}

impl<I: hash::Hash + cmp::Eq, P: Default + Ord> WeightedAvgModel<I, P> {
    pub fn evaluate(&self, inp: &I) -> Option<f64> {
        self.memory.get(inp).map(|entry| entry.value.mean())
    }

    pub fn learn(&mut self, input: I, value: f64, priority: P) {
        self.weighted_learn(input, value, 1.0, priority)
    }

    pub fn weighted_learn(&mut self, input: I, value: f64, weight: f64, priority: P) {
        self.weighted_learn_with_decay(input, value, weight, priority, 1.0)
    }

    pub fn weighted_learn_with_decay(
        &mut self,
        input: I,
        value: f64,
        weight: f64,
        priority: P,
        decay: f64,
    ) {
        let entry = self.memory.entry(input).or_default();

        match priority.cmp(&entry.priority) {
            Ordering::Less => {}
            Ordering::Equal => {
                entry.value.scale(decay);
                entry.value.add_sample(value, weight);
            }
            Ordering::Greater => {
                entry.value = weighted_avg::WeightedAvg::with_value(value, weight);
                entry.priority = priority;
            }
        }
    }
}

impl<I: Display + std::cmp::Ord, P> Display for WeightedAvgModel<I, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.memory
            .iter()
            .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
            .try_for_each(|(key, value)| {
                key.fmt(f)?;
                f.write_str(": ")?;
                value.value.fmt(f)?;
                f.write_char('\n')
            })
    }
}
