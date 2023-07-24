use std::{fmt, hash};

use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::board::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
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
struct MaxIter<'a, T> {
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
        let max;

        (self.iters, max) = self
            .iters
            .drain(..)
            .map(|mut iter| (iter.next(), iter))
            .fold((Vec::new(), None), |(mut iters, mut max), (next, iter)| {
                match max.cmp(&next) {
                    // reset
                    std::cmp::Ordering::Less => {
                        iters = vec![iter];
                        max = next;
                    }

                    // add
                    std::cmp::Ordering::Equal => iters.push(iter),

                    // ignore
                    std::cmp::Ordering::Greater => (),
                }

                (iters, max)
            });

        max
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Dihedral<const N: u8> {
    pub flip: bool,
    pub rot: u8,
}

impl<const N: u8> Default for Dihedral<N> {
    fn default() -> Self {
        Self::id()
    }
}

impl<const N: u8> Dihedral<N> {
    pub fn id() -> Self {
        Self {
            flip: false,
            rot: 0,
        }
    }

    pub fn op(&self, other: &Self) -> Self {
        Self {
            flip: self.flip != other.flip,
            rot: if other.flip { N - self.rot } else { self.rot } + other.rot % N,
        }
    }

    pub fn inverse(&self) -> Self {
        Self {
            rot: if self.flip { N - self.rot } else { self.rot },
            flip: self.flip,
        }
    }

    pub fn all() -> DihedralGenerator<N> {
        DihedralGenerator::default()
    }
}

impl Dihedral<4> {
    pub fn is_horizontal(&self) -> bool {
        (self.rot % 2 == 0) ^ self.flip
    }
}

impl<const N: u8> fmt::Display for Dihedral<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = format!("r{}", self.rot);
        if self.flip {
            res = "-".to_string() + &res;
        }

        f.pad(&res)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct DihedralGenerator<const N: u8> {
    pub next: Option<Dihedral<N>>,
}

impl<const N: u8> Iterator for DihedralGenerator<N> {
    type Item = Dihedral<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;

        self.next = match current {
            Dihedral { flip: true, rot } if rot + 1 >= N => None,

            Dihedral { flip: true, rot } => Some(Dihedral {
                flip: true,
                rot: rot + 1,
            }),

            Dihedral { flip: false, rot } if rot + 1 >= N => Some(Dihedral { flip: true, rot: 0 }),

            Dihedral { flip: false, rot } => Some(Dihedral {
                flip: false,
                rot: rot + 1,
            }),
        };

        Some(current)
    }
}

impl<const N: u8> Default for DihedralGenerator<N> {
    fn default() -> Self {
        Self {
            next: Some(Default::default()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MaybeRev<Iter> {
    Forward(Iter),
    Backward(Iter),
}

impl<Iter> MaybeRev<Iter> {
    pub fn new(iter: Iter, forward: bool) -> Self {
        if forward {
            Self::Forward(iter)
        } else {
            Self::Backward(iter)
        }
    }
}

impl<Iter> Iterator for MaybeRev<Iter>
where
    Iter: Iterator + DoubleEndedIterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MaybeRev::Forward(iter) => iter.next(),
            MaybeRev::Backward(iter) => iter.next_back(),
        }
    }
}

impl<Iter> DoubleEndedIterator for MaybeRev<Iter>
where
    Iter: Iterator + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            MaybeRev::Forward(iter) => iter.next_back(),
            MaybeRev::Backward(iter) => iter.next(),
        }
    }
}

fn gen_indices(
    rows: usize,
    cols: usize,
    perm: Dihedral<4>,
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    let row_inc = (perm.rot + 1) % 4 <= 1;
    let col_inc = perm.rot % 4 <= 1;

    let row_range = MaybeRev::new(0..rows, row_inc);
    let col_range = MaybeRev::new(0..cols, col_inc);

    let column_first = perm.is_horizontal();

    if column_first {
        Box::new(row_range.cartesian_product(col_range))
    } else {
        Box::new(col_range.cartesian_product(row_range).map(|(c, r)| (r, c)))
    }
}

#[derive(Clone, Debug, Eq)]
pub struct SymmetricBoard<const ROWS: usize, const COLS: usize> {
    max_perm: Board<ROWS, COLS>,
}

impl<const ROWS: usize, const COLS: usize> Default for SymmetricBoard<ROWS, COLS> {
    fn default() -> Self {
        Board::default().into()
    }
}

impl<const ROWS: usize, const COLS: usize> PartialEq for SymmetricBoard<ROWS, COLS> {
    fn eq(&self, other: &Self) -> bool {
        self.max_perm == other.max_perm
    }
}

impl<const ROWS: usize, const COLS: usize> hash::Hash for SymmetricBoard<ROWS, COLS> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.max_perm.hash(state)
    }
}

impl<const ROWS: usize, const COLS: usize> fmt::Display for SymmetricBoard<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.max_perm.fmt(f)
    }
}

impl<const ROWS: usize, const COLS: usize> From<Board<ROWS, COLS>> for SymmetricBoard<ROWS, COLS> {
    fn from(board: Board<ROWS, COLS>) -> Self {
        board.cells.into()
    }
}

impl<const ROWS: usize, const COLS: usize> From<[[u8; COLS]; ROWS]> for SymmetricBoard<ROWS, COLS> {
    fn from(board: [[u8; COLS]; ROWS]) -> Self {
        let mut max_iter = MaxIter::new();

        let product = (0..ROWS).cartesian_product(0..COLS);
        max_iter.add_iter(product.clone().map(|(r, c)| board[r][c]));
        max_iter.add_iter(product.clone().map(|(r, c)| board[ROWS - r - 1][c]));
        max_iter.add_iter(product.clone().map(|(r, c)| board[r][COLS - c - 1]));
        max_iter.add_iter(product.map(|(r, c)| board[ROWS - r - 1][COLS - c - 1]));

        if ROWS == COLS {
            let product = (0..COLS).cartesian_product(0..ROWS);
            max_iter.add_iter(product.clone().map(|(c, r)| board[r][c]));
            max_iter.add_iter(product.clone().map(|(c, r)| board[ROWS - r - 1][c]));
            max_iter.add_iter(product.clone().map(|(c, r)| board[r][COLS - c - 1]));
            max_iter.add_iter(product.map(|(c, r)| board[ROWS - r - 1][COLS - c - 1]));
        }

        let cells: [[u8; COLS]; ROWS] = max_iter
            .collect::<Vec<_>>()
            .chunks_exact(COLS)
            .map(|row| <[u8; COLS]>::try_from(row).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        SymmetricBoard {
            max_perm: cells.into(),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_dihedral_gen() {
        assert!(Dihedral::<4>::all().count() == 8);
        assert!(Dihedral::<3>::all().count() == 6);
        assert!(Dihedral::<2>::all().count() == 4);
        assert!(Dihedral::<1>::all().count() == 2);
    }

    #[test]
    fn test_board_eq() {
        let corner1 = SymmetricBoard::from([[0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0]]);
        let corner2 = SymmetricBoard::from([[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
        assert_eq!(corner1, corner2);
    }

    #[test]
    fn test_symmetry() {
        let board = Board::from([[2, 1], [0, 0]]);
        let sym_board = SymmetricBoard::from(board.clone());
        println!("{sym_board}");
    }
}
