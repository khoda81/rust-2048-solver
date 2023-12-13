use std::num::NonZeroU8;

type Block = NonZeroU8;
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Row<const SIZE: usize>([Option<Block>; SIZE]);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum SwipeResult {
    #[default]
    UnChanged,
    Changed {
        score: u32,
    },
}

impl SwipeResult {
    fn handle_merge(&mut self, merged_block: Block) {
        let new_score = 2u32.pow(merged_block.get().into());
        match self {
            SwipeResult::UnChanged => *self = SwipeResult::Changed { score: new_score },
            SwipeResult::Changed { score } => *score += new_score,
        }
    }

    fn handle_move(&mut self) {
        match self {
            SwipeResult::UnChanged => *self = SwipeResult::Changed { score: 0 },
            SwipeResult::Changed { .. } => {}
        }
    }
}

impl<const SIZE: usize> Row<SIZE> {
    #[inline(always)]
    pub fn swipe_left(&mut self) -> SwipeResult {
        let mut last_pos = 0;
        let mut result = SwipeResult::UnChanged;
        let mut cells = self.0;

        for i in 1..cells.len() {
            match cells[i] {
                None => continue,

                // merge
                Some(block) if cells[last_pos] == Some(block) => {
                    if let Some(merged_block) = block.checked_add(1) {
                        cells[last_pos] = Some(merged_block);
                        cells[i] = None;

                        if let Some(block) = cells[last_pos] {
                            result.handle_merge(block)
                        }

                        last_pos += 1;
                        continue;
                    }
                }

                _ => {}
            }

            // move
            if cells[last_pos].is_some() {
                last_pos += 1;
            }

            if last_pos != i {
                cells.swap(last_pos, i);
                result.handle_move()
            }
        }

        self.0 = cells;

        result
    }
}

impl<const SIZE: usize> From<[u8; SIZE]> for Row<SIZE> {
    fn from(value: [u8; SIZE]) -> Self {
        Row(value.map(Block::new))
    }
}
impl<const SIZE: usize> From<Row<SIZE>> for [u8; SIZE] {
    fn from(value: Row<SIZE>) -> Self {
        value.0.map(|cell| cell.map_or(0, |block| block.get()))
    }
}

#[inline(always)]
pub fn shift_row<const SIZE: usize>(cells: &mut [u8; SIZE]) -> bool {
    let mut row = Row::from(*cells);
    let result = row.swipe_left();
    *cells = row.into();

    !matches!(result, SwipeResult::UnChanged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_row() {
        let mut row = Row::from([0, 0, 0, 0]);
        row.swipe_left();
        assert_eq!(row, Row::from([0, 0, 0, 0]));

        let mut row = Row::from([0, 0, 0, 1]);
        row.swipe_left();
        assert_eq!(row, Row::from([1, 0, 0, 0]));

        let mut row = Row::from([0, 0, 1, 0]);
        row.swipe_left();
        assert_eq!(row, Row::from([1, 0, 0, 0]));

        let mut row = Row::from([0, 0, 1, 1]);
        row.swipe_left();
        assert_eq!(row, Row::from([2, 0, 0, 0]));

        let mut row = Row::from([1, 0, 1, 1]);
        row.swipe_left();
        assert_eq!(row, Row::from([2, 1, 0, 0]));

        let mut row = Row::from([1, 1, 1, 0]);
        row.swipe_left();
        assert_eq!(row, Row::from([2, 1, 0, 0]));

        let mut row = Row::from([0, 2, 1, 2]);
        row.swipe_left();
        assert_eq!(row, Row::from([2, 1, 2, 0]));

        let mut row = Row::from([1, 3, 2, 4]);
        row.swipe_left();
        assert_eq!(row, Row::from([1, 3, 2, 4]));

        let mut row = Row::from([1, 2, 2, 3]);
        row.swipe_left();
        assert_eq!(row, Row::from([1, 3, 3, 0]));

        let mut row = Row::from([1, 0, 1, 2]);
        row.swipe_left();
        assert_eq!(row, Row::from([2, 2, 0, 0]));

        let mut row = Row::from([1, 1, 1, 1]);
        row.swipe_left();
        assert_eq!(row, Row::from([2, 2, 0, 0]));

        let mut row = Row::from([2, 1, 0, 1]);
        row.swipe_left();
        assert_eq!(row, Row::from([2, 2, 0, 0]));

        let mut row = Row::from([]);
        row.swipe_left();
        assert_eq!(row, Row::from([]));

        let mut row = Row::from([1]);
        row.swipe_left();
        assert_eq!(row, Row::from([1]));
    }
}
