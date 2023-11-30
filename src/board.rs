use std::fmt::{self, Write};

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
    pub cells: [[u8; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize> Default for Board<COLS, ROWS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const COLS: usize, const ROWS: usize> Board<COLS, ROWS> {
    pub fn new() -> Self {
        [[0; COLS]; ROWS].into()
    }

    #[inline(always)]
    pub fn count_empty(&self) -> usize {
        self.cells.iter().flatten().filter(|&c| c == &0).count()
    }

    #[inline(always)]
    pub fn spawns(&self) -> impl IntoIterator<Item = (Self, f64)> + '_ {
        self.cells
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                std::iter::repeat(i)
                    .zip(0_usize..)
                    .zip(row)
                    .filter_map(|(pos, cell)| (cell == 0).then_some(pos))
            })
            .flat_map(|(i, j)| {
                [
                    {
                        let mut new_board_1 = self.cells;
                        new_board_1[i][j] = 1;
                        (new_board_1.into(), 2.)
                    },
                    {
                        let mut new_board_2 = self.cells;
                        new_board_2[i][j] = 2;
                        (new_board_2.into(), 1.)
                    },
                ]
            })
    }

    #[inline(always)]
    pub fn random_spawn(&self) -> Self {
        let mut options: Vec<_> = self.spawns().into_iter().collect();
        let weights = options.iter().map(|item| item.1);
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();
        let index = dist.sample(&mut rng);
        options.swap_remove(index).0
    }

    #[inline(always)]
    pub fn swipe_left(&mut self) -> bool {
        self.cells.iter_mut().map(shift_row).max().unwrap_or(false)
    }

    #[inline(always)]
    pub fn swipe_right(&mut self) -> bool {
        self.cells
            .iter_mut()
            .map(|row| {
                row.reverse();
                let moved = shift_row(row);
                row.reverse();
                moved
            })
            .max()
            .unwrap_or(false)
    }

    #[inline(always)]
    pub fn swipe_up(&mut self) -> bool {
        (0..COLS)
            .map(|i| {
                let mut row = [0; ROWS];
                (0..ROWS).for_each(|j| {
                    row[j] = self.cells[j][i];
                });

                let moved = shift_row(&mut row);

                (0..ROWS).for_each(|j| {
                    self.cells[j][i] = row[j];
                });

                moved
            })
            .max()
            .unwrap_or(false)
    }

    #[inline(always)]
    pub fn swipe_down(&mut self) -> bool {
        (0..COLS)
            .map(|i| {
                let mut row = [0; ROWS];
                (0..ROWS).for_each(|j| {
                    row[j] = self.cells[j][i];
                });

                row.reverse();
                let moved = shift_row(&mut row);
                row.reverse();

                (0..ROWS).for_each(|j| {
                    self.cells[j][i] = row[j];
                });
                moved
            })
            .max()
            .unwrap_or(false)
    }

    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        (0..ROWS - 1).all(|i| (0..COLS).all(|j| self.cells[i][j] != self.cells[i + 1][j]))
            && (0..ROWS).all(|i| (0..COLS - 1).all(|j| self.cells[i][j] != self.cells[i][j + 1]))
            && self.cells.iter().flatten().all(|&x| x != 0)
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
}

impl<const COLS: usize, const ROWS: usize> fmt::Display for Board<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.cells.iter().peekable();

        while let Some(row) = iter.next() {
            format_row(row, f)?;
            if iter.peek().is_some() {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn format_row(last_row: &[u8], f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    last_row.iter().try_for_each(|cell| {
        f.write_char(match cell {
            0 => b'.',
            1..=9 => b'0' + cell,
            _ => cell - 10 + b'a',
        } as char)?;

        f.write_char(' ')
    })
}

impl<const COLS: usize, const ROWS: usize> From<[[u8; COLS]; ROWS]> for Board<COLS, ROWS> {
    fn from(cells: [[u8; COLS]; ROWS]) -> Self {
        Self { cells }
    }
}

impl<const COLS: usize, const ROWS: usize> From<Board<COLS, ROWS>> for [[u8; COLS]; ROWS] {
    fn from(board: Board<COLS, ROWS>) -> Self {
        board.cells
    }
}
