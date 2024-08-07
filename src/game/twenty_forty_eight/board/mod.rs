pub mod fast_swipe;

use crate::accumulator::fraction::Weighted;
use std::fmt::Write as _;
use std::marker::PhantomData;
use std::num::NonZeroU8;
use std::ops::{Deref, DerefMut};
use std::simd::{cmp::SimdPartialEq as _, u8x16};
use std::{array, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl crate::game::Discrete for Direction {
    fn iter() -> impl Iterator<Item = Self> {
        Directions::default()
    }
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

#[derive(Debug, Clone)]
struct Directions {
    pub current: Option<Direction>,
}

impl Default for Directions {
    fn default() -> Self {
        Directions {
            current: Some(Direction::Up),
        }
    }
}

impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.current? {
            Direction::Up => Some(Direction::Down),
            Direction::Down => Some(Direction::Left),
            Direction::Left => Some(Direction::Right),
            Direction::Right => None,
        };

        std::mem::replace(&mut self.current, next)
    }
}

// TODO: Implement From for {i32, u32, u8, i8, ...}
// TODO: Change this to an enum of 1/2
#[derive(Debug, Clone, Copy)]
pub struct Weight(NonZeroU8);
impl Weight {
    fn new(n: u8) -> Option<Self> {
        NonZeroU8::new(n).map(Weight)
    }
}
impl Deref for Weight {
    type Target = NonZeroU8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Weight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<Weight> for f32 {
    fn from(value: Weight) -> Self {
        f32::from(value.0.get())
    }
}
impl From<Weight> for f64 {
    fn from(value: Weight) -> Self {
        f64::from(value.0.get())
    }
}
pub type Cell = u8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cells<const COLS: usize, const ROWS: usize> {
    pub cells: [[Cell; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize> Cells<COLS, ROWS> {
    pub fn new() -> Self {
        Self::from_cells([[0; COLS]; ROWS])
    }

    pub fn from_cells(cells: impl Into<[[Cell; COLS]; ROWS]>) -> Self {
        Self {
            cells: cells.into(),
        }
    }

    pub fn count_empty(&self) -> usize {
        // NOTE: This is optimized to use SIMD.
        self.into_iter().flatten().filter(|&c| c == 0).count()
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
        use std::array::from_fn;

        Cells::from_cells(from_fn(|row_idx| from_fn(|col_idx| self[col_idx][row_idx])))
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
        let bytes = unsafe { std::mem::transmute::<[[u8; 4]; 4], [u8; 16]>(self.cells) };
        u128::from_le_bytes(bytes)
    }

    fn as_simd(self) -> u8x16 {
        // SAFETY: we know the slice is 16 bytes and has the same layout
        let bytes = unsafe { std::mem::transmute::<[[u8; 4]; 4], [u8; 16]>(self.cells) };
        u8x16::from_array(bytes)
    }
}

impl<const COLS: usize, const ROWS: usize> std::hash::Hash for Cells<COLS, ROWS> {
    #[inline(never)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(cells) = <dyn std::any::Any>::downcast_ref::<Cells<4, 4>>(self) {
            return cells.as_u128().hash(state);
        }

        let cells = self.cells.as_flattened();
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
        Cells { cells }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("COLS*ROWS should be {} but its ({ROWS}*{COLS}={})", std::mem::size_of::<u128>(), ROWS * COLS)]
pub struct SizeMismatch<const COLS: usize, const ROWS: usize>(pub PhantomData<[[(); COLS]; ROWS]>);
impl<const COLS: usize, const ROWS: usize> TryFrom<u128> for Cells<COLS, ROWS> {
    type Error = SizeMismatch<COLS, ROWS>;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let bytes = value.to_le_bytes();
        if COLS * ROWS == bytes.len() {
            // SAFETY: A [u8; COLS * ROWS] has the same size and layout as [[u8; COLS]; ROWS]
            let bytes: [[u8; COLS]; ROWS] = unsafe { *bytes.as_ptr().cast() };
            Ok(Self::from_cells(bytes))
        } else {
            Err(SizeMismatch(PhantomData))
        }
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

#[derive(Debug)]
pub struct Spawns<const COLS: usize, const ROWS: usize> {
    cells: Cells<COLS, ROWS>,
    mask: Cells<COLS, ROWS>,
}

impl<const COLS: usize, const ROWS: usize> Spawns<COLS, ROWS> {
    pub fn new(cells: Cells<COLS, ROWS>) -> Self {
        let mut mask = Cells::new();
        mask.cells[0][0] = 2;
        Spawns { cells, mask }
    }

    pub fn empty() -> Self {
        Spawns {
            cells: Cells::new(),
            mask: Cells::new(),
        }
    }
}

impl Spawns<4, 4> {
    fn fast_next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            let simd_cells = self.cells.as_simd();
            let simd_masks = self.mask.as_simd();

            if simd_masks == u8x16::splat(0) {
                break;
            }

            let has_1 = (simd_masks & u8x16::splat(1)) != u8x16::splat(0);
            let weight = Weight::new(if has_1 { 2 } else { 1 });

            let new_cells = (simd_cells | simd_masks).to_array();

            // SAFETY: A [Cell; 16] has the same layout as [[Cell; 4]; 4].
            let new_cells: [[Cell; 4]; 4] = unsafe { std::mem::transmute(new_cells) };
            let new_cells = Cells::from_cells(new_cells);

            let zero_mask = simd_masks.simd_eq(u8x16::splat(0));
            let last_element_zero = zero_mask.test(15);
            let mut sub_arr = [0; 16];
            sub_arr[15] = if last_element_zero { 0 } else { 1 };
            let simd_masks = simd_masks - u8x16::from_array(sub_arr);

            // Rotate bytes to the right
            let mask_bytes = simd_masks.rotate_elements_right::<1>();

            // SAFETY: A [Cell; 16] has the same layout as [[Cell; 4]; 4].
            let unflatten_bytes: [[Cell; 4]; 4] = unsafe { std::mem::transmute(mask_bytes) };
            self.mask = Cells::from_cells(unflatten_bytes);

            let cell_zero = simd_cells.simd_eq(u8x16::splat(0));
            if (zero_mask | cell_zero).all() {
                let value = new_cells;
                return weight.map(|weight| Weighted { value, weight });
            }
        }

        None
    }
}

impl<const COLS: usize, const ROWS: usize> Iterator for Spawns<COLS, ROWS> {
    type Item = Weighted<Cells<COLS, ROWS>, Weight>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(spawns) = <dyn std::any::Any>::downcast_mut::<Spawns<4, 4>>(self) {
            let result = spawns.fast_next();
            return *<dyn std::any::Any>::downcast_ref(&result).unwrap();
        }

        // TODO:
        todo!()
    }

    // TODO: Implement size_hint?
}

// TODO: Write a macro for creating boards
#[cfg(test)]
mod test_board {
    use super::{Cells, Spawns, Weight};
    use crate::accumulator::fraction::Weighted;
    use itertools::Itertools;
    use std::sync::Once;

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

    static INIT: Once = Once::new();

    /// Setup function that is only run once, even if called multiple times.
    fn setup() {
        INIT.call_once(|| {
            env_logger::Builder::new()
                .filter_level(log::LevelFilter::Trace)
                .parse_default_env()
                .init();
        });
    }

    #[test]
    fn test_swipe() {
        setup();
        let reversed = |mut row: [u8; 4]| {
            row.reverse();
            row
        };

        for (inp, expected_out) in TEST_CASES.iter().copied() {
            let inp = Cells::from_cells(inp);
            let expected_out = Cells::from_cells(expected_out);

            {
                let mut cells = inp;
                assert_eq!(cells.swipe_left(), inp != expected_out, "Input: {inp:?}");
                assert_eq!(cells, expected_out, "Input: {inp:?}");
            }
            {
                let inp = Cells::from_cells(inp.map(reversed));
                let expected_out = Cells::from_cells(expected_out.map(reversed));

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
                let inp = Cells::from_cells(inp.map(reversed)).transposed();
                let expected_out = Cells::from_cells(expected_out.map(reversed)).transposed();

                let mut cells = inp;
                assert_eq!(cells.swipe_down(), inp != expected_out, "Input: {inp:?}");
                assert_eq!(cells, expected_out, "Input: {inp:?}");
            }
        }
    }

    #[test]
    fn test_spawns() {
        setup();
        for &cells in TEST_CASES.iter().flat_map(|(i, o)| [i, o]) {
            let cells = Cells::from_cells(cells);
            let mut fast_spawns = Spawns::new(cells).sorted_by_key(|spawns| spawns.value.cells);

            let mut slow_spawns = cells
                .into_iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.into_iter()
                        .enumerate()
                        .filter_map(move |(j, cell)| (cell == 0).then_some((i, j)))
                })
                .flat_map(move |(i, j)| {
                    [(1, 2), (2, 1)].map(|(weight, cell)| {
                        let mut new_board = cells;
                        new_board.cells[i][j] = cell;
                        Weighted::new_weighted(new_board, Weight::new(weight).unwrap())
                    })
                })
                .sorted_by_key(|spawns| spawns.value.cells);

            for (fast, slow) in (&mut fast_spawns).zip(&mut slow_spawns) {
                assert_eq!(
                    fast.value, fast.value,
                    "Fast cell:\n{}\nis not equal to slow cell:\n{}",
                    fast.value, fast.value
                );
                assert_eq!(
                    fast.value, slow.value,
                    "Weights are not equal for: \n{}",
                    fast.value
                );
            }

            if let Some(weighted) = fast_spawns.next() {
                panic!(
                    "Fast spawn yielded an extra spawn: weight={:?}\n{}",
                    weighted.weight, weighted.value
                );
            }

            if let Some(weighted) = slow_spawns.next() {
                panic!(
                    "Slow spawn yielded an extra spawn: weight={:?}\n{}",
                    weighted.weight, weighted.value
                );
            }
        }
    }

    // TODO: Test count empty
}
