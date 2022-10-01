/// Moves the cells to the left and merges adjacent cells with the same value.
///
/// # Examples
///
/// ```
/// use rust_2048_solver::rs_2048::move_row;
/// let mut row = vec![2, 2, 2, 2];;
/// assert_eq!(shift_row(row), [3, 3, 0, 0]);
/// ```

pub fn shift_row<const SIZE: usize>(row: &[u8; SIZE]) -> [u8; SIZE] {
    let mut row = row.clone();
    let mut last_pos = 0;

    for i in 1..row.len() {
        if row[i] == 0 {
            continue;
        }

        if row[i] == row[last_pos] {
            // merge
            row[last_pos] += 1;
            row[i] = 0;
            last_pos += 1;
        } else {
            // move
            if row[last_pos] != 0 {
                last_pos += 1;
            }

            row.swap(last_pos, i);
        }
    }

    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_row() {
        let row = [0, 0, 0, 0];
        assert_eq!(shift_row(&row), [0, 0, 0, 0]);

        let row = [0, 0, 0, 1];
        assert_eq!(shift_row(&row), [1, 0, 0, 0]);

        let row = [0, 0, 1, 1];
        assert_eq!(shift_row(&row), [2, 0, 0, 0]);

        let row = [1, 0, 1, 1];
        assert_eq!(shift_row(&row), [2, 1, 0, 0]);

        let row = [1, 1, 1, 0];
        assert_eq!(shift_row(&row), [2, 1, 0, 0]);

        let row = [0, 2, 1, 2];
        assert_eq!(shift_row(&row), [2, 1, 2, 0]);

        let row = [1, 3, 2, 4];
        assert_eq!(shift_row(&row), [1, 3, 2, 4]);

        let row = [1, 2, 2, 3];
        assert_eq!(shift_row(&row), [1, 3, 3, 0]);

        let row = [1, 0, 1, 2];
        assert_eq!(shift_row(&row), [2, 2, 0, 0]);

        let row = [0, 0, 1, 0];
        assert_eq!(shift_row(&row), [1, 0, 0, 0]);

        let row = [1, 1, 1, 1];
        assert_eq!(shift_row(&row), [2, 2, 0, 0]);

        let row = [];
        assert_eq!(shift_row(&row), []);

        let row = [1];
        assert_eq!(shift_row(&row), [1]);
    }
}
