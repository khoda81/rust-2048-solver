// TODO rename to models

use std::{
    cmp,
    collections::HashMap,
    fmt::{Display, Write},
    hash,
    ops::{AddAssign, Deref, DerefMut},
};

use itertools::Itertools;

pub mod preprocessor;
pub mod prioritized;
pub mod weighted;

#[derive(Clone, Debug)]
pub struct AccumulationModel<K, V> {
    pub memory: HashMap<K, V>,
}

impl<K, V> Default for AccumulationModel<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> AccumulationModel<K, V> {
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }
}

impl<K, V> AccumulationModel<K, V>
where
    K: hash::Hash + cmp::Eq,
    V: Default + AddAssign,
{
    pub fn insert(&mut self, key: K, value: V) {
        self.memory.entry(key).or_default().add_assign(value)
    }
}

impl<K: Display + std::cmp::Ord, V: Display> Display for AccumulationModel<K, V> {
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

impl<K, V> Deref for AccumulationModel<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}

impl<K, V> DerefMut for AccumulationModel<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.memory
    }
}
