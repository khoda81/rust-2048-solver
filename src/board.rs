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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Board<const COLS: usize, const ROWS: usize> {
    pub cells: [[u8; COLS]; ROWS],
}

impl<const COLS: usize, const ROWS: usize> Default for Board<COLS, ROWS> {
    fn default() -> Self {
        [[0; COLS]; ROWS].into()
    }
}

impl<const COLS: usize, const ROWS: usize> Board<COLS, ROWS> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count_empty(&self) -> usize {
        self.cells.iter().flatten().filter(|&c| c == &0).count()
    }

    pub fn spawns(&self) -> Vec<(Self, f64)> {
        let mut boards = Vec::new();

        for (i, row) in self.cells.into_iter().enumerate() {
            for (j, _) in row.into_iter().enumerate().filter(|&c| c.1 == 0) {
                let mut new_board = self.cells;
                new_board[i][j] = 1;
                boards.push((new_board.into(), 2.));

                let mut new_board = self.cells;
                new_board[i][j] = 2;
                boards.push((new_board.into(), 1.));
            }
        }

        boards
    }

    pub fn random_spawn(&self) -> Self {
        let mut options = self.spawns();
        let weights = options.iter().map(|item| item.1);
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();
        let index = dist.sample(&mut rng);
        options.swap_remove(index).0
    }

    pub fn move_left(&mut self) -> bool {
        self.cells.iter_mut().map(shift_row).max().unwrap_or(false)
    }

    pub fn move_right(&mut self) -> bool {
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

    pub fn move_up(&mut self) -> bool {
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

    pub fn move_down(&mut self) -> bool {
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

    pub fn is_lost(&self) -> bool {
        (0..ROWS - 1).all(|i| (0..COLS).all(|j| self.cells[i][j] != self.cells[i + 1][j]))
            && (0..ROWS).all(|i| (0..COLS - 1).all(|j| self.cells[i][j] != self.cells[i][j + 1]))
            && self.cells.iter().flatten().all(|&x| x != 0)
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

impl<const COLS: usize, const ROWS: usize> fmt::Display for Board<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.cells.iter();
        if let Some(last_row) = iter.next_back() {
            for row in iter {
                row.iter().try_for_each(|cell| {
                    if cell == &0 {
                        write!(f, " . ")
                    } else {
                        write!(f, "{cell:2?} ")
                    }
                })?;

                writeln!(f)?;
            }

            last_row.iter().try_for_each(|cell| {
                if cell == &0 {
                    write!(f, " . ")
                } else {
                    write!(f, "{cell:2?} ")
                }
            })?;
        }

        Ok(())
    }
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
