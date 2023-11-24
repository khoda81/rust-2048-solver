use std::{cmp, collections::HashMap, hash};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct EvaluationEntry<P> {
    pub value_sum: f64,
    pub num_samples: u32,
    pub priority: P,
}

impl<P: Ord> EvaluationEntry<P> {
    pub fn update_with(&mut self, value: f64, priority: P) {
        match priority.cmp(&self.priority) {
            cmp::Ordering::Less => {}
            cmp::Ordering::Equal => {
                // Add value to samples
                self.num_samples += 1;
                self.value_sum += value;
            }
            cmp::Ordering::Greater => {
                // New sample overrides previous values
                self.num_samples = 1;
                self.value_sum = value;
                self.priority = priority;
            }
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value_sum / self.num_samples as f64
    }
}

#[derive(Clone, Debug, Default)]
pub struct Model<K, P> {
    pub evaluation_memory: HashMap<K, EvaluationEntry<P>>,
}

impl<K, P> Model<K, P> {
    pub fn new() -> Self {
        Self {
            evaluation_memory: HashMap::new(),
        }
    }
}

impl<K: hash::Hash + cmp::Eq, P: Default + Ord> Model<K, P> {
    pub fn evaluate(&self, key: &K) -> Option<&EvaluationEntry<P>> {
        self.evaluation_memory.get(key)
    }

    pub fn learn(&mut self, key: K, value: f64, priority: P) {
        self.evaluation_memory
            .entry(key)
            .or_default()
            .update_with(value, priority);
    }
}
#[cfg(test)]
mod test_model {
    use super::*;

    #[test]
    fn test_evaluation_entry_update() {
        let mut entry = EvaluationEntry::default();
        entry.update_with(5.0, 1);

        assert_eq!(entry.num_samples, 1);
        assert_eq!(entry.value_sum, 5.0);
        assert_eq!(entry.priority, 1);

        entry.update_with(8.0, 2);

        assert_eq!(entry.num_samples, 1);
        assert_eq!(entry.value_sum, 8.0);
        assert_eq!(entry.priority, 2);

        entry.update_with(3.0, 2);

        assert_eq!(entry.num_samples, 2);
        assert_eq!(entry.value_sum, 11.0);
        assert_eq!(entry.priority, 2);
    }

    #[test]
    fn test_evaluation_entry_get_value() {
        let entry = EvaluationEntry {
            value_sum: 15.0,
            num_samples: 3,
            priority: 2,
        };

        assert_eq!(entry.get_value(), 5.0);
    }

    #[test]
    fn test_model_evaluate() {
        let mut model = Model::new();
        let obs = "observation";

        assert_eq!(model.evaluate(&obs), None);

        model.learn(obs, 8.0, 2);

        if let Some(entry) = model.evaluate(&obs) {
            assert_eq!(entry.get_value(), 8.0);
            assert_eq!(entry.num_samples, 1);
            assert_eq!(entry.priority, 2);
        } else {
            panic!("Expected Some, got None.");
        }
    }

    #[test]
    fn test_model_learn() {
        let mut model = Model::new();
        let obs1 = "observation1";
        let obs2 = "observation2";

        model.learn(obs1, 5.0, 1);

        if let Some(entry) = model.evaluate(&obs1) {
            assert_eq!(entry.get_value(), 5.0);
            assert_eq!(entry.num_samples, 1);
            assert_eq!(entry.priority, 1);
        } else {
            panic!("Expected Some, got None.");
        }

        // Learning a different observation
        model.learn(obs2, 10.0, 3);

        if let Some(entry) = model.evaluate(&obs2) {
            assert_eq!(entry.get_value(), 10.0);
            assert_eq!(entry.num_samples, 1);
            assert_eq!(entry.priority, 3);
        } else {
            panic!("Expected Some, got None.");
        }
    }
}
