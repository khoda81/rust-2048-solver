use std::{
    self, cmp,
    collections::HashMap,
    fmt::{Debug, Display, Write},
    time::Duration,
};

use itertools::Itertools;

use crate::bots::{self, heuristic, mean_max::MeanMax, model::AccumulationModel};

/// Iterator is the lexicographic maximum of all the iterators added to it.
///
/// # Examples
///
/// ```
/// use rust_2048_solver::utils::MaxIter;
///
/// let a = [1, 2, 2, 0];
/// let b = [1, 2, 1, 0];
/// let c = [1, 2, 2, 0, 0];
///
/// let mut max_iter = MaxIter::new();
/// max_iter.add_iter(a.into_iter());
/// max_iter.add_iter(b.into_iter());
/// max_iter.add_iter(c.into_iter());
///
/// assert_eq!(max_iter.collect::<Vec<_>>(), vec![1, 2, 2, 0, 0]);
/// ```
///
#[derive(Default)]
pub struct MaxIter<'a, T> {
    pub iters: Vec<Box<dyn Iterator<Item = T> + 'a>>,
}

impl<'a, T> MaxIter<'a, T> {
    pub fn new() -> Self {
        MaxIter { iters: Vec::new() }
    }

    pub fn add_iter(&mut self, iter: impl Iterator<Item = T> + 'a) {
        self.iters.push(Box::new(iter));
    }
}

impl<T: Ord> Iterator for MaxIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let max;

        (self.iters, max) = self
            .iters
            .drain(..)
            .map(|mut iter| (iter.next(), iter))
            .fold((Vec::new(), None), |(mut iters, mut max), (next, iter)| {
                match max.cmp(&next) {
                    // reset
                    cmp::Ordering::Less => {
                        iters = vec![iter];
                        max = next;
                    }

                    // add
                    cmp::Ordering::Equal => iters.push(iter),

                    // ignore
                    cmp::Ordering::Greater => (),
                }

                (iters, max)
            });

        max
    }
}

pub fn show_fill_percent(ai: &MeanMax<4, 4>) {
    let capacity = ai.evaluation_cache.cap().get();
    let filled = ai.evaluation_cache.len();
    let fill_percent = (filled * 100) as f64 / capacity as f64;
    println!("Evaluation cache is {fill_percent:.2?}% filled",);
}

pub fn print_model<K: Debug + Ord, V: Display>(model: &AccumulationModel<K, V>) {
    model
        .memory
        .iter()
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .for_each(|(key, value)| println!("{key:2?}: {value}"));
}

pub fn print_lookup<const ROWS: usize, const COLS: usize>(
    ai: &bots::mean_max::MeanMax<ROWS, COLS>,
) {
    let mut new_lookup = heuristic::get_lookup().clone();

    for (key, eval) in ai.model.memory.iter() {
        new_lookup.insert(*key, eval.average_value());
    }

    show_map(&new_lookup);
}

pub fn show_map<V: std::fmt::Debug>(map: &HashMap<heuristic::PreprocessedBoard, V>) {
    for (key, value) in map
        .iter()
        .sorted_by_key(|(&(empty, max), _eval)| (max, empty))
    {
        println!("map.insert({key:2?}, {value:?});");
        // println!("data[{key:?}] = {value}");
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Signed<T> {
    Positive(T),
    Negative(T),
}

impl<T: Display> Display for Signed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            Signed::Positive(inner) => inner,
            Signed::Negative(inner) => {
                f.write_char('-')?;
                inner
            }
        };

        inner.fmt(f)
    }
}

impl<T: Debug> Debug for Signed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            Signed::Positive(inner) => inner,
            Signed::Negative(inner) => {
                f.write_char('-')?;
                inner
            }
        };

        inner.fmt(f)
    }
}

pub fn get_signed_duration(seconds: f64) -> Signed<Duration> {
    let abs_duration = Duration::from_secs_f64(seconds.abs());
    if seconds.is_sign_positive() {
        Signed::Positive(abs_duration)
    } else {
        Signed::Negative(abs_duration)
    }
}
