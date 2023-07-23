use std::{
    fmt::{Debug, Formatter, Result},
    hash::{Hash, Hasher},
};

use rand::{
    distributions::{Distribution, Standard, WeightedIndex},
    Rng,
};

use crate::shift_row::shift_row;

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

#[derive(Clone, Eq)]
pub struct Board<const ROWS: usize, const COLS: usize> {
    board: [[u8; COLS]; ROWS],
    // max_perm: Rc<RefCell<Option<Board<ROWS, COLS>>>>,
}

impl<const ROWS: usize, const COLS: usize> Default for Board<ROWS, COLS> {
    fn default() -> Self {
        Board {
            board: [[0; COLS]; ROWS],
            // max_perm: Default::default(),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Board<ROWS, COLS> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = [u8; COLS]> + '_ {
        self.board.iter().cloned()
    }

    pub fn cols_iter(&self) -> impl Iterator<Item = [u8; ROWS]> + '_ {
        (0..COLS).map(move |col| {
            let mut new_row = [0; ROWS];
            for row in self.rows_iter().enumerate() {
                new_row[col] = row.1[col];
            }

            new_row
        })
    }

    pub fn get_max_perm(&self) -> impl Iterator<Item = &u8> {
        let mut max_iter = MaxIter::new();

        let product = itertools::iproduct!(0..ROWS, 0..COLS);
        max_iter.add_iter(product.clone().map(|(r, c)| &self.board[r][c]));
        max_iter.add_iter(product.clone().map(|(r, c)| &self.board[ROWS - r - 1][c]));
        max_iter.add_iter(product.clone().map(|(r, c)| &self.board[r][COLS - c - 1]));
        max_iter.add_iter(product.map(|(r, c)| &self.board[ROWS - r - 1][COLS - c - 1]));

        let product = itertools::iproduct!(0..COLS, 0..ROWS);
        max_iter.add_iter(product.clone().map(|(c, r)| &self.board[r][c]));
        max_iter.add_iter(product.clone().map(|(c, r)| &self.board[ROWS - r - 1][c]));
        max_iter.add_iter(product.clone().map(|(c, r)| &self.board[r][COLS - c - 1]));
        max_iter.add_iter(product.map(|(c, r)| &self.board[ROWS - r - 1][COLS - c - 1]));

        max_iter
    }

    pub fn spawns(&self) -> Vec<(Self, f64)> {
        let mut boards = Vec::new();

        for (i, row) in self.board.into_iter().enumerate() {
            for (j, _) in row.into_iter().enumerate().filter(|&c| c.1 == 0) {
                let mut new_board = self.board;
                new_board[i][j] = 1;
                boards.push((new_board.into(), 2.));

                let mut new_board = self.board;
                new_board[i][j] = 2;
                boards.push((new_board.into(), 1.));
            }
        }

        boards
    }

    pub fn random_spawn(&mut self) -> Self {
        let mut options = self.spawns();
        let weights = options.iter().map(|item| item.1);
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();
        let index = dist.sample(&mut rng);
        options.swap_remove(index).0
    }

    pub fn move_left(&mut self) -> bool {
        self.board
            .iter_mut()
            .filter_map(|row| shift_row(row).then_some(()))
            .count()
            .gt(&0)
    }

    pub fn move_right(&mut self) -> bool {
        self.board
            .iter_mut()
            .filter_map(|row| {
                row.reverse();
                let moved = shift_row(row);
                row.reverse();
                moved.then_some(())
            })
            .count()
            .gt(&0)
    }

    pub fn move_up(&mut self) -> bool {
        let mut new_board = self.board;
        let mut moved = false;
        for i in 0..COLS {
            let mut row = [0; ROWS];
            for j in 0..ROWS {
                row[j] = new_board[j][i];
            }

            moved |= shift_row(&mut row);

            for j in 0..ROWS {
                new_board[j][i] = row[j];
            }
        }

        self.board = new_board;
        moved
    }

    pub fn move_down(&mut self) -> bool {
        let mut new_board = self.board;
        let mut moved = false;
        for i in 0..COLS {
            let mut row = [0; ROWS];
            for j in 0..ROWS {
                row[j] = new_board[j][i];
            }

            row.reverse();
            moved |= shift_row(&mut row);
            row.reverse();

            for j in 0..ROWS {
                new_board[j][i] = row[j];
            }
        }

        self.board = new_board;
        moved
    }

    pub fn is_lost(&self) -> bool {
        (0..ROWS - 1).all(|i| (0..COLS).all(|j| self.board[i][j] != self.board[i + 1][j]))
            && (0..ROWS).all(|i| (0..COLS - 1).all(|j| self.board[i][j] != self.board[i][j + 1]))
            && self.board.iter().flatten().all(|&x| x != 0)
    }

    pub fn swipe(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
        }
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

impl<const ROWS: usize, const COLS: usize> Debug for Board<ROWS, COLS> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in self.board.iter() {
            for cell in row.iter() {
                if cell == &0 {
                    write!(f, " . ")?;
                } else {
                    write!(f, "{:2?} ", cell)?;
                }
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

impl<const ROWS: usize, const COLS: usize> From<Board<ROWS, COLS>> for [[u8; COLS]; ROWS] {
    fn from(val: Board<ROWS, COLS>) -> Self {
        val.board
    }
}
