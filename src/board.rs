pub mod fast_swipe;

use rand::distributions::{Distribution, WeightedIndex};
use rand::seq::SliceRandom;
use std::fmt::Write as _;
use std::hash::Hash;
use std::{
    array, fmt,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: &'static [Self] = &[
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => f.write_char('↑'),
            Direction::Down => f.write_char('↓'),
            Direction::Left => f.write_char('←'),
            Direction::Right => f.write_char('→'),
        }
    }
}

pub type Weight = u8;
pub type Cell = u8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cells<const COLS: usize, const ROWS: usize> {
    pub cells: [[Cell; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize> Cells<COLS, ROWS> {
    pub fn new() -> Self {
        [[0; COLS]; ROWS].into()
    }

    pub fn count_empty(&self) -> usize {
        // NOTE: This is optimized to use SIMD.
        self.into_iter().flatten().filter(|&c| c == 0).count()
    }

    pub fn spawns(&self) -> impl Iterator<Item = (Self, Weight)> {
        self.into_spawns()
    }

    #[deprecated]
    pub fn into_spawns(self) -> impl Iterator<Item = (Self, Weight)> {
        // PERF: We should be able to represent the state of this iterator using a single 128 bit mask
        self.into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(j, cell)| (cell == 0).then_some((i, j)))
            })
            .flat_map(move |(i, j)| {
                [(1, 2), (2, 1)].map(|(cell, weight)| {
                    let mut new_board = self;
                    new_board.cells[i][j] = cell;
                    (new_board, weight)
                })
            })
    }

    pub fn iter_spawns_random(self) -> impl Iterator<Item = (Self, Weight)> {
        // PERF: This can probably be optimized
        let mut positions = Vec::with_capacity(16);
        positions.extend(self.into_iter().enumerate().flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(j, cell)| (cell == 0).then_some((i, j)))
        }));

        positions.shuffle(&mut rand::thread_rng());
        positions.into_iter().flat_map(move |(i, j)| {
            [(1, 2), (2, 1)].map(|(cell, weight)| {
                let mut new_board = self;
                new_board.cells[i][j] = cell;
                (new_board, weight)
            })
        })
    }

    pub fn random_spawn(&self) -> Self {
        // PERF: Don't generate all the possible states beforehand
        let options: Vec<_> = self.spawns().collect();
        let weights = options.iter().map(|(_board, weight)| weight);
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();
        let index = dist.sample(&mut rng);
        options[index].0
    }

    pub fn swipe_left(&mut self) -> bool {
        self.iter_mut()
            .map(fast_swipe::swipe_left)
            .fold(false, bool::max)
    }

    pub fn swipe_right(&mut self) -> bool {
        self.iter_mut()
            .map(fast_swipe::swipe_right)
            .fold(false, bool::max)
    }

    pub fn swipe_up(&mut self) -> bool {
        self.columns()
            .enumerate()
            .map(|(i, mut column)| {
                let moved = fast_swipe::swipe_left(&mut column);
                column.into_iter().enumerate().for_each(|(j, cell)| {
                    self[j][i] = cell;
                });

                moved
            })
            .fold(false, bool::max)
    }

    pub fn swipe_down(&mut self) -> bool {
        self.columns()
            .enumerate()
            .map(|(i, mut column)| {
                column.reverse();
                let moved = fast_swipe::swipe_left(&mut column);
                column.into_iter().rev().enumerate().for_each(|(j, cell)| {
                    self[j][i] = cell;
                });

                moved
            })
            .fold(false, bool::max)
    }

    #[must_use]
    pub fn is_lost(&self) -> bool {
        !self.has_move()
    }

    #[must_use]
    pub fn has_move(&self) -> bool {
        self.iter().flatten().any(|&x| x == 0)
            || (0..ROWS - 1).any(|i| (0..COLS).any(|j| self[i][j] == self[i + 1][j]))
            || (0..ROWS).any(|i| (0..COLS - 1).any(|j| self[i][j] == self[i][j + 1]))
    }

    pub fn swipe(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.swipe_left(),
            Direction::Right => self.swipe_right(),
            Direction::Up => self.swipe_up(),
            Direction::Down => self.swipe_down(),
        }
    }

    #[must_use]
    pub fn swiped(mut self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Left => self.swipe_left(),
            Direction::Right => self.swipe_right(),
            Direction::Up => self.swipe_up(),
            Direction::Down => self.swipe_down(),
        }
        .then_some(self)
    }

    pub fn transposed(self) -> Cells<ROWS, COLS> {
        let mut transposed = [[0; ROWS]; COLS];
        for row in 0..ROWS {
            for col in 0..COLS {
                transposed[col][row] = self[row][col];
            }
        }

        Cells::from(transposed)
    }

    pub fn columns(self) -> impl Iterator<Item = [Cell; ROWS]> {
        (0..COLS).map(move |i| array::from_fn(|j| self[j][i]))
    }

    pub fn rows(self) -> impl Iterator<Item = [Cell; COLS]> {
        self.into_iter()
    }
}

impl Cells<4, 4> {
    pub fn as_u128(self) -> u128 {
        // SAFETY: we know the slice is 16 bytes and has the same layout
        let bytes = unsafe { *self.cells.as_ptr().cast::<[u8; 16]>() };
        u128::from_le_bytes(bytes)
    }
}

impl<const COLS: usize, const ROWS: usize> std::hash::Hash for Cells<COLS, ROWS> {
    #[inline(never)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(cells) = (self as &dyn std::any::Any).downcast_ref::<Cells<4, 4>>() {
            return cells.as_u128().hash(state);
        }

        let cells = self.cells.flatten();
        let chunks = cells.chunks_exact(8);

        let remainder = chunks.remainder();

        let mut last_chunk = [0; 8];
        last_chunk[..remainder.len()].copy_from_slice(remainder);
        let remainder = (!remainder.is_empty()).then_some(last_chunk.as_slice());

        chunks
            .chain(remainder)
            .map(|chunk| {
                // SAFETY: this is safe since using [`<[_]>::chunks_exact`] with size 8
                unsafe { chunk.try_into().unwrap_unchecked() }
            })
            .map(u64::from_ne_bytes)
            .for_each(|chunk| chunk.hash(state));
    }
}

impl<const COLS: usize, const ROWS: usize> Default for Cells<COLS, ROWS> {
    fn default() -> Self {
        Self::new()
    }
}

fn format_row(last_row: &[Cell], f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    for cell in last_row {
        let cell_char = match cell {
            0 => b'.',
            1..=9 => cell + b'0',
            _ => cell - 10 + b'a',
        } as char;

        f.write_char(cell_char)?;
        f.write_char(' ')?;
    }

    Ok(())
}

impl<const COLS: usize, const ROWS: usize> fmt::Display for Cells<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.iter() {
            format_row(row, f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const COLS: usize, const ROWS: usize> From<[[Cell; COLS]; ROWS]> for Cells<COLS, ROWS> {
    fn from(cells: [[Cell; COLS]; ROWS]) -> Self {
        Self { cells }
    }
}

impl<const COLS: usize, const ROWS: usize> From<Cells<COLS, ROWS>> for [[Cell; COLS]; ROWS] {
    fn from(board: Cells<COLS, ROWS>) -> Self {
        *board
    }
}

impl<const COLS: usize, const ROWS: usize> Deref for Cells<COLS, ROWS> {
    type Target = [[Cell; COLS]; ROWS];

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl<const COLS: usize, const ROWS: usize> DerefMut for Cells<COLS, ROWS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

// TODO: Write a macro for creating boards
#[cfg(test)]
mod test_board {
    use super::Cells;

    type TestCase = ([[u8; 4]; 4], [[u8; 4]; 4]);

    const TEST_CASES: &[TestCase] = &[
        (
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        ),
        (
            [[1, 0, 0, 0], [0, 0, 0, 0], [3, 0, 0, 0], [0, 0, 0, 0]],
            [[1, 0, 0, 0], [0, 0, 0, 0], [3, 0, 0, 0], [0, 0, 0, 0]],
        ),
        (
            [[1, 0, 1, 0], [0, 2, 1, 1], [0, 0, 0, 0], [1, 2, 1, 0]],
            [[2, 0, 0, 0], [2, 2, 0, 0], [0, 0, 0, 0], [1, 2, 1, 0]],
        ),
        (
            [[0, 0, 1, 1], [0, 0, 9, 1], [0, 1, 1, 3], [1, 6, 2, 5]],
            [[2, 0, 0, 0], [9, 1, 0, 0], [2, 3, 0, 0], [1, 6, 2, 5]],
        ),
        (
            [[2, 0, 0, 2], [1, 3, 0, 0], [6, 1, 5, 0], [1, 2, 9, 2]],
            [[3, 0, 0, 0], [1, 3, 0, 0], [6, 1, 5, 0], [1, 2, 9, 2]],
        ),
        (
            [[1, 6, 3, 2], [0, 0, 9, 1], [0, 0, 0, 3], [0, 2, 0, 5]],
            [[1, 6, 3, 2], [9, 1, 0, 0], [3, 0, 0, 0], [2, 5, 0, 0]],
        ),
        (
            [[0, 0, 0, 0], [1, 0, 0, 0], [0, 1, 3, 9], [3, 6, 1, 5]],
            [[0, 0, 0, 0], [1, 0, 0, 0], [1, 3, 9, 0], [3, 6, 1, 5]],
        ),
        (
            [[0, 0, 0, 2], [2, 0, 1, 1], [0, 0, 9, 3], [2, 6, 1, 5]],
            [[2, 0, 0, 0], [2, 2, 0, 0], [9, 3, 0, 0], [2, 6, 1, 5]],
        ),
        (
            [[1, 6, 1, 5], [1, 2, 5, 1], [1, 3, 4, 1], [6, 0, 0, 0]],
            [[1, 6, 1, 5], [1, 2, 5, 1], [1, 3, 4, 1], [6, 0, 0, 0]],
        ),
        (
            [[2, 7, 3, 1], [3, 5, 7, 0], [2, 7, 2, 1], [1, 0, 0, 0]],
            [[2, 7, 3, 1], [3, 5, 7, 0], [2, 7, 2, 1], [1, 0, 0, 0]],
        ),
    ];

    #[test]
    fn test_swipe() {
        let reversed = |mut row: [u8; 4]| {
            row.reverse();
            row
        };

        for (inp, expected_out) in TEST_CASES.iter().copied() {
            let inp = Cells::from(inp);
            let expected_out = Cells::from(expected_out);

            {
                let mut cells = inp;
                assert_eq!(cells.swipe_left(), inp != expected_out, "Input: {inp:?}");
                assert_eq!(cells, expected_out, "Input: {inp:?}");
            }
            {
                let inp = Cells::from(inp.map(reversed));
                let expected_out = Cells::from(expected_out.map(reversed));

                let mut cells = inp;
                assert_eq!(cells.swipe_right(), inp != expected_out, "Input: {inp:?}");
                assert_eq!(cells, expected_out, "Input: {inp:?}");
            }
            {
                let inp = inp.transposed();
                let expected_out = expected_out.transposed();

                let mut cells = inp;
                assert_eq!(cells.swipe_up(), inp != expected_out, "Input: {inp:?}");
                assert_eq!(cells, expected_out, "Input: {inp:?}");
            }
            {
                let inp = Cells::from(inp.map(reversed)).transposed();
                let expected_out = Cells::from(expected_out.map(reversed)).transposed();

                let mut cells = inp;
                assert_eq!(cells.swipe_down(), inp != expected_out, "Input: {inp:?}");
                assert_eq!(cells, expected_out, "Input: {inp:?}");
            }
        }
    }

    // TODO: Test count empty
    // TODO: Test iter spawns
}
