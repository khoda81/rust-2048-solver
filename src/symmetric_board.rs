use std::{fmt, hash};

use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::{board, utils};

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

#[derive(Clone, Debug, Eq)]
pub struct SymmetricBoard<const COLS: usize, const ROWS: usize> {
    max_board: board::Board<COLS, ROWS>,
}

impl<const COLS: usize, const ROWS: usize> Default for SymmetricBoard<COLS, ROWS> {
    fn default() -> Self {
        board::Board::default().into()
    }
}

impl<const COLS: usize, const ROWS: usize> PartialEq for SymmetricBoard<COLS, ROWS> {
    fn eq(&self, other: &Self) -> bool {
        self.max_board == other.max_board
    }
}

impl<const COLS: usize, const ROWS: usize> hash::Hash for SymmetricBoard<COLS, ROWS> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.max_board.hash(state)
    }
}

impl<const COLS: usize, const ROWS: usize> fmt::Display for SymmetricBoard<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.max_board.fmt(f)
    }
}

impl<const COLS: usize, const ROWS: usize> From<board::Board<COLS, ROWS>>
    for SymmetricBoard<COLS, ROWS>
{
    fn from(board: board::Board<COLS, ROWS>) -> Self {
        board.cells.into()
    }
}

impl<const COLS: usize, const ROWS: usize> From<[[u8; COLS]; ROWS]> for SymmetricBoard<COLS, ROWS> {
    fn from(board: [[u8; COLS]; ROWS]) -> Self {
        let mut max_iter = utils::MaxIter::new();

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
            max_board: cells.into(),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_board_eq() {
        let corner1 = SymmetricBoard::from([[0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0]]);
        let corner2 = SymmetricBoard::from([[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
        assert_eq!(corner1, corner2);
    }

    #[test]
    fn test_symmetry() {
        // let board = board::Board::from([[2, 1], [0, 0]]);
        let board = board::Board::from([
            // board
            [1, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [2, 0, 0, 0],
        ]);

        let sym_board = SymmetricBoard::from(board);
        println!("{sym_board}");
    }
}
