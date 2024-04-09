// TODO rename to models

use itertools::Itertools;
use std::{
    cmp,
    collections::{hash_map::Entry, HashMap},
    fmt::{Display, Write},
    hash,
    ops::AddAssign,
};

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
}

impl<K, V> Accumulator<K, V>
where
    K: hash::Hash + cmp::Eq,
    V: AddAssign,
{
    pub fn accumulate(&mut self, key: K, value: V) {
        match self.memory.entry(key) {
            Entry::Occupied(mut occupied_entry) => occupied_entry.get_mut().add_assign(value),
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(value);
            }
        }
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
