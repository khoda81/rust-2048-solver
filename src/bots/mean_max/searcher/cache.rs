use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
};

pub struct PriorityCache<K, V, P> {
    priorities: BTreeSet<(P, K)>,
    values: HashMap<K, V>,
    capacity: usize,
}

impl<K, V, P> PriorityCache<K, V, P> {
    pub fn new(capacity: usize) -> Self {
        Self {
            priorities: BTreeSet::new(),
            values: HashMap::new(),
            capacity,
        }
    }
}

impl<K, V, P> PriorityCache<K, V, P>
where
    K: Hash + Ord + Clone,
    P: Ord,
{
    pub fn put(&mut self, key: K, value: V, priority: P) {
        while self.values.len() >= self.capacity {
            let Some((_priority, key)) = self.priorities.pop_first() else {
                return;
            };
            self.values.remove(&key);
        }

        self.priorities.insert((priority, key.clone()));
        self.values.insert(key, value);
    }
}

impl<K, V, P> PriorityCache<K, V, P>
where
    K: Hash + Eq,
{
    pub fn get(&self, key: &K) -> Option<&V> {
        self.values.get(key)
    }
}
