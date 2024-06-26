use std::{
    cmp,
    fmt::{self, Debug, Display, Write as _},
    hash::Hash,
    time::Duration,
};

use itertools::Itertools;

use crate::accumulator;

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

pub fn show_fill_percent<K, V>(cache: &lru::LruCache<K, V>)
where
    K: Eq + Hash,
{
    let capacity = cache.cap().get();
    let filled = cache.len();
    let fill_percent = (filled * 100) as f64 / capacity as f64;
    println!("Evaluation cache is {fill_percent:.2?}% filled",);
}

pub fn print_model<K: Debug + Ord, V: Display>(model: &accumulator::Accumulator<K, V>) {
    model
        .memory
        .iter()
        .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
        .for_each(|(key, value)| println!("{key:2?}: {value}"));
}

// TODO: make these generic

// pub fn print_lookup<S>(ai: &MeanMax<S, heuristic::PreprocessedBoard>) {
//     let mut new_lookup = heuristic::get_lookup().clone();
//
//     for (key, eval) in ai.model.memory.iter() {
//         new_lookup.insert(*key, eval.weighted_average());
//     }
//
//     show_map(&new_lookup);
// }
//
// pub fn show_map<V: Debug>(map: &HashMap<heuristic::PreprocessedBoard, V>) {
//     for (key, value) in map
//         .iter()
//         .sorted_by_key(|(&(empty, max, ordered), _eval)| (max, empty, ordered))
//     {
//         println!("map.insert({key:2?}, {value:?});");
//         // println!("data[{key:?}] = {value}");
//     }
// }

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Signed<T> {
    Positive(T),
    Negative(T),
}

impl<T: Display> Display for Signed<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Format the duration as human readable
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct HumanDuration(pub Duration);

impl Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Change to 3 digit precision.
        // TODO: Make precision configurable?

        let duration = self.0;
        let seconds = duration.as_secs();
        let nanos = duration.as_nanos();

        let buf = if nanos < 1_00 {
            // 1ns-99ns
            format!("{}ns", nanos)
        } else if nanos < 9_950 {
            // 1.0µs-9.9µs
            format!("{:.1}µs", nanos as f32 / 1_000.0)
        } else if nanos < 99_500 {
            // 10µs-99µs
            format!("{}µs", nanos / 1_000)
        } else if nanos < 9_500_000 {
            // 0.1ms-9.9ms
            format!("{:.1}ms", nanos as f32 / 1_000_000.0)
        } else if nanos < 99_500_000 {
            // 10ms-99ms
            format!("{}ms", nanos / 1_000_000)
        } else if seconds < 10 {
            // 0.1s-9.9s
            format!("{:.1}s", nanos as f32 / 1_000_000_000.0)
        } else if seconds < 60 {
            // 10s-59s
            format!("{}s", seconds)
        } else if seconds < 597 {
            // 1.0m-9.9m
            format!("{:.1}m", seconds as f32 / 60.0)
        } else if seconds < 3_600 {
            // 10m-59m
            format!("{}m", seconds / 60)
        } else if seconds < 35820 {
            // 1.0h-9.9h
            format!("{:.1}h", seconds as f32 / 3_600.0)
        } else {
            // 1h-...
            format!("{}h", seconds / 3_600)
        };

        f.pad(&buf)
    }
}
