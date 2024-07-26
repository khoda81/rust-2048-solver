use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::{cmp, hash, ops};

pub mod fraction;
pub mod prioritized;

#[derive(Clone, Debug)]
pub struct Accumulator<K, V> {
    pub memory: HashMap<K, V>,
}

impl<K, V> Default for Accumulator<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Accumulator<K, V> {
    pub fn new() -> Self {
        Self {
            // PERF: Replace this with Vec binary search.
            memory: HashMap::new(),
        }
    }

    pub fn accumulate<T>(&mut self, key: K, value: T)
    where
        K: hash::Hash + cmp::Eq,
        V: Default + ops::AddAssign<T>,
    {
        self.memory.entry(key).or_default().add_assign(value)
    }
}

impl<K: Display + std::cmp::Ord, V: Display> Display for Accumulator<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.memory
            .iter()
            .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
            .try_for_each(|(key, value)| {
                key.fmt(f)?;
                f.write_str(": ")?;
                value.fmt(f)?;
                f.write_char('\n')
            })
    }
}
