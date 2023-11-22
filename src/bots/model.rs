use std::{cmp, collections::HashMap, hash};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct EvaluationEntry {
    value_sum: f64,
    num_samples: u32,
    level: u32,
}

pub struct Model<Obs, F: Fn(&Obs) -> f64> {
    pub evaluation_memory: HashMap<Obs, EvaluationEntry>,
    pub heuristic: F,
}

impl<Obs: hash::Hash + cmp::Eq, F: Fn(&Obs) -> f64> Model<Obs, F> {
    pub fn evaluate(&self, obs: &Obs) -> f64 {
        let entry = self
            .evaluation_memory
            .get(obs)
            .map(|entry| entry.value_sum / entry.num_samples as f64);

        entry.unwrap_or_else(|| (self.heuristic)(obs))
    }

    pub fn learn(&mut self, obs: Obs, value: f64, level: u32) {
        let my_entry = EvaluationEntry {
            value_sum: value,
            num_samples: 1,
            level,
        };

        let new_entry = self
            .evaluation_memory
            .get(&obs)
            .and_then(|entry| match entry.level.cmp(&level) {
                cmp::Ordering::Less => None,
                cmp::Ordering::Equal => Some(EvaluationEntry {
                    value_sum: my_entry.value_sum + value,
                    num_samples: my_entry.num_samples + 1,
                    level,
                }),
                cmp::Ordering::Greater => Some(*entry),
            })
            .unwrap_or(my_entry);

        self.evaluation_memory.insert(obs, new_entry);
    }
}
