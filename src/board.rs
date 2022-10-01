use std::hash::{Hash, Hasher};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Iterator is the lexicographic maximum of all the iterators added to it.
///
/// # Examples
///
/// ```
/// use rust_2048_solver::board::MaxIter;
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
    iters: Vec<Box<dyn Iterator<Item = T> + 'a>>,
}

impl<'a, T> MaxIter<'a, T> {
    pub fn new() -> Self {
        MaxIter { iters: Vec::new() }
    }

    pub fn add_iter(&mut self, perm: impl Iterator<Item = T> + 'a) {
        self.iters.push(Box::new(perm));
    }
}

impl<T: Ord> Iterator for MaxIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (new_iters, max) =
            self.iters
                .drain(..)
                .fold((Vec::new(), None), |(mut acc, max), mut iter| {
                    let next = iter.next();
                    if next > max {
                        return (vec![iter], next);
                    } else if next == max {
                        acc.push(iter);
                    }

                    (acc, max)
                });

        self.iters = new_iters;
        max
    }
}

#[derive(Clone)]
pub struct Board<const ROWS: usize, const COLS: usize> {
    board: [[u8; COLS]; ROWS],
    // max_perm: Rc<RefCell<Option<Board<ROWS, COLS>>>>,
}

impl<const ROWS: usize, const COLS: usize> Board<ROWS, COLS> {
    pub fn new() -> Self {
        Board {
            board: [[0; COLS]; ROWS],
            // max_perm: Default::default(),
        }
    }

    pub fn get_max_perm(&self) -> impl Iterator<Item = &u8> {
        let mut max_iter = MaxIter::new();
        max_iter.add_iter(self.board.iter().flatten()); // normal
        max_iter.add_iter(self.board.iter().map(|row| row.iter().rev()).flatten()); // hflip
        max_iter.add_iter(
            self.board
                .iter()
                .rev()
                .map(|row| row.iter().rev())
                .flatten(),
        ); // 180rot
        max_iter.add_iter(self.board.iter().rev().flatten()); // vflip

        max_iter
    }
}

impl<const R1: usize, const C1: usize, const R2: usize, const C2: usize> PartialEq<Board<R2, C2>>
    for Board<R1, C1>
{
    fn eq(&self, other: &Board<R2, C2>) -> bool {
        self.get_max_perm().eq(other.get_max_perm())
    }
}

impl<const ROWS: usize, const COLS: usize> Hash for Board<ROWS, COLS> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.get_max_perm() {
            c.hash(state);
        }
    }
}

impl<const ROWS: usize, const COLS: usize> std::fmt::Debug for Board<ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.board.iter() {
            for cell in row.iter() {
                write!(f, "{:2?} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const ROWS: usize, const COLS: usize> From<[[u8; COLS]; ROWS]> for Board<ROWS, COLS> {
    fn from(board: [[u8; COLS]; ROWS]) -> Self {
        Self {
            board,
            // max_perm: Default::default(),
        }
    }
}
