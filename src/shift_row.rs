/// Moves the cells to the left and merges adjacent cells with the same value.
///
/// # Examples
///
/// ```
/// use rust_2048_solver::shift_row::shift_row;
/// let mut row = [2, 2, 2, 2];;
/// shift_row(&mut row);
/// assert_eq!(row, [3, 3, 0, 0]);
/// ```
pub fn shift_row<const SIZE: usize>(row: &mut [u8; SIZE]) -> bool {
    let mut last_pos = 0;
    let mut changed = false;

    for i in 1..row.len() {
        if row[i] == 0 {
            continue;
        }

        if row[i] == row[last_pos] {
            // merge
            row[last_pos] += 1;
            row[i] = 0;
            last_pos += 1;

            changed = true;
        } else {
            // move
            if row[last_pos] != 0 {
                last_pos += 1;
            }

            row.swap(last_pos, i);
            changed = changed || last_pos != i;
        }
    }

    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_row() {
        let mut row = [0, 0, 0, 0];
        shift_row(&mut row);
        assert_eq!(row, [0, 0, 0, 0]);

        let mut row = [0, 0, 0, 1];
        shift_row(&mut row);
        assert_eq!(row, [1, 0, 0, 0]);

        let mut row = [0, 0, 1, 0];
        shift_row(&mut row);
        assert_eq!(row, [1, 0, 0, 0]);

        let mut row = [0, 0, 1, 1];
        shift_row(&mut row);
        assert_eq!(row, [2, 0, 0, 0]);

        let mut row = [1, 0, 1, 1];
        shift_row(&mut row);
        assert_eq!(row, [2, 1, 0, 0]);

        let mut row = [1, 1, 1, 0];
        shift_row(&mut row);
        assert_eq!(row, [2, 1, 0, 0]);

        let mut row = [0, 2, 1, 2];
        shift_row(&mut row);
        assert_eq!(row, [2, 1, 2, 0]);

        let mut row = [1, 3, 2, 4];
        shift_row(&mut row);
        assert_eq!(row, [1, 3, 2, 4]);

        let mut row = [1, 2, 2, 3];
        shift_row(&mut row);
        assert_eq!(row, [1, 3, 3, 0]);

        let mut row = [1, 0, 1, 2];
        shift_row(&mut row);
        assert_eq!(row, [2, 2, 0, 0]);

        let mut row = [1, 1, 1, 1];
        shift_row(&mut row);
        assert_eq!(row, [2, 2, 0, 0]);

        let mut row = [2, 1, 0, 1];
        shift_row(&mut row);
        assert_eq!(row, [2, 2, 0, 0]);

        let mut row = [];
        shift_row(&mut row);
        assert_eq!(row, []);

        let mut row = [1];
        shift_row(&mut row);
        assert_eq!(row, [1]);
    }
}
