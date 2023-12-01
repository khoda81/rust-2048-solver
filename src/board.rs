use std::fmt::Write as _;
use std::{
    array, fmt,
    ops::{Deref, DerefMut},
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

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arrow = match self {
            Direction::Up => '↑',
            Direction::Down => '↓',
            Direction::Left => '←',
            Direction::Right => '→',
        };

        f.write_char(arrow)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Board<const COLS: usize, const ROWS: usize> {
    pub cells: [[Cell; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize> Default for Board<COLS, ROWS> {
    fn default() -> Self {
        Self::new()
    }
}

pub type Weight = u8;
pub type Cell = u8;

#[allow(clippy::unnecessary_fold)]
impl<const COLS: usize, const ROWS: usize> Board<COLS, ROWS> {
    pub fn new() -> Self {
        [[0; COLS]; ROWS].into()
    }

    #[inline(always)]
    pub fn count_empty(&self) -> usize {
        self.iter().flatten().filter(|&c| c == &0).count()
    }

    #[inline(always)]
    pub fn spawns(&self) -> impl Iterator<Item = (Self, Weight)> + '_ {
        self.into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(move |(j, cell)| (cell == 0).then_some((i, j)))
            })
            .flat_map(|(i, j)| {
                [(1, 2), (2, 1)].map(|(cell, weight)| {
                    let mut new_board = self.cells;
                    new_board[i][j] = cell;
                    (new_board.into(), weight)
                })
            })
    }

    #[inline(always)]
    pub fn random_spawn(&self) -> Self {
        let options: Vec<_> = self.spawns().collect();
        let weights = options.iter().map(|(_board, weight)| weight);
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();
        let index = dist.sample(&mut rng);
        options[index].0
    }

    #[inline(always)]
    pub fn swipe_left(&mut self) -> bool {
        self.iter_mut().map(shift_row).fold(false, bool::max)
    }

    #[inline(always)]
    pub fn swipe_right(&mut self) -> bool {
        self.iter_mut()
            .map(|row| {
                row.reverse();
                let moved = shift_row(row);
                row.reverse();
                moved
            })
            .fold(false, bool::max)
    }

    #[inline(always)]
    pub fn swipe_up(&mut self) -> bool {
        self.columns()
            .enumerate()
            .map(|(i, mut column)| {
                let moved = shift_row(&mut column);
                column.into_iter().enumerate().for_each(|(j, cell)| {
                    self[j][i] = cell;
                });

                moved
            })
            .fold(false, bool::max)
    }

    #[inline(always)]
    pub fn swipe_down(&mut self) -> bool {
        self.columns()
            .enumerate()
            .map(|(i, mut column)| {
                column.reverse();
                let moved = shift_row(&mut column);
                column.into_iter().rev().enumerate().for_each(|(j, cell)| {
                    self[j][i] = cell;
                });

                moved
            })
            .fold(false, bool::max)
    }

    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        !self.has_move()
    }

    pub fn has_move(&self) -> bool {
        self.iter().flatten().any(|&x| x == 0)
            || (0..ROWS - 1).any(|i| (0..COLS).any(|j| self[i][j] == self[i + 1][j]))
            || (0..ROWS).any(|i| (0..COLS - 1).any(|j| self[i][j] == self[i][j + 1]))
    }

    #[inline(always)]
    pub fn swipe(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.swipe_left(),
            Direction::Right => self.swipe_right(),
            Direction::Up => self.swipe_up(),
            Direction::Down => self.swipe_down(),
        }
    }

    pub fn columns(self) -> impl Iterator<Item = [Cell; ROWS]> {
        (0..COLS).map(move |i| array::from_fn(|j| self[j][i]))
    }

    pub fn rows(self) -> impl Iterator<Item = [Cell; COLS]> {
        self.into_iter()
    }
}

impl<const COLS: usize, const ROWS: usize> fmt::Display for Board<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print the first row without nextline
        if let Some(row) = self.first() {
            format_row(row, f)?;
        }

        for row in &self[1..] {
            writeln!(f)?;
            format_row(row, f)?;
        }

        Ok(())
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

impl<const COLS: usize, const ROWS: usize> From<[[Cell; COLS]; ROWS]> for Board<COLS, ROWS> {
    fn from(cells: [[Cell; COLS]; ROWS]) -> Self {
        Self { cells }
    }
}

impl<const COLS: usize, const ROWS: usize> From<Board<COLS, ROWS>> for [[Cell; COLS]; ROWS] {
    fn from(board: Board<COLS, ROWS>) -> Self {
        *board
    }
}

impl<const COLS: usize, const ROWS: usize> Deref for Board<COLS, ROWS> {
    type Target = [[Cell; COLS]; ROWS];

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl<const COLS: usize, const ROWS: usize> DerefMut for Board<COLS, ROWS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}
