use std::cmp;

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
