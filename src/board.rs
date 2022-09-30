use std::{iter::{Map,}, slice::{IterMut, self}};


#[derive(Clone)]
pub struct Board<const ROWS: usize, const COLS: usize> {
    board: [[u8; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Board<ROWS, COLS> {
    pub fn new()  -> Self {
        Board {
            board: [[0; COLS]; ROWS],
        }
    }

    pub(crate) fn from_array(array: [[u8; COLS]; ROWS])  -> Self {
        Board { board: array }
    }

    pub fn rows(&self) -> impl Iterator<Item = &[u8; COLS]> {
        self.board.iter()
    }

    pub fn hflip(&self) -> Self {
        let mut board = self.board.clone();
        for row in board.iter_mut() {
            row.reverse()
        }

        Board { board }
    }

    pub fn transpose(&self) -> Board<COLS, ROWS> {
        let mut board = [[0; ROWS]; COLS];
        for i in 0..ROWS {
            for j in 0..COLS {
                board[j][i] = self.board[i][j];
            }
        }

        Board { board }
    }
}

impl<const R1: usize, const C1: usize, const R2: usize, const C2: usize> PartialEq<Board<R2, C2>>
    for Board<R1, C1>
where
    [[u8; C1]; R1]: PartialEq<[[u8; C2]; R2]>,
{
    fn eq(&self, other: &Board<R2, C2>) -> bool {
        self.board == other.board
    }
}

impl<const ROWS: usize, const COLS: usize> std::fmt::Debug for Board<{ ROWS }, { COLS }> {
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

// from iterator
impl<const ROWS: usize, const COLS: usize> FromIterator<[u8; COLS]> for Board<ROWS, COLS> {
    fn from_iter<I: IntoIterator<Item = [u8; COLS]>>(iter: I) -> Self {
        let mut board = [[0; COLS]; ROWS];
        for (i, row) in iter.into_iter().enumerate() {
            board[i] = row;
        }

        Board { board }
    }
}
