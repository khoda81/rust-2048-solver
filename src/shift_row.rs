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
    #[inline]
    fn handle_merge(&mut self, merged_block: Block) {
        let new_score = 2u32.pow(merged_block.get().into());
        match self {
            SwipeResult::UnChanged => *self = SwipeResult::Changed { score: new_score },
            SwipeResult::Changed { score } => *score += new_score,
        }
    }

    #[inline]
    fn handle_move(&mut self) {
        match self {
            SwipeResult::UnChanged => *self = SwipeResult::Changed { score: 0 },
            SwipeResult::Changed { .. } => {}
        }
    }
}

impl<const SIZE: usize> Row<SIZE> {
    #[inline(always)]
    fn swipe_4_fast(mut bytes: u32) -> u32 {
        // Early return if all are zeros
        // if bytes == 0 { return bytes; }

        // Move block[3] to the right if block[2] is empty
        if bytes & 0xFF0000 == 0 {
            // PERF: use fast bit merge
            bytes = (bytes >> 8) & !0xFFFF | bytes & 0xFFFF;
        }

        // Move block[3, 2] to the right if block[1] is empty
        if bytes & 0xFF00 == 0 {
            // PERF: use fast bit merge
            bytes = (bytes >> 8) & !0xFF | bytes & 0xFF;
        }

        // Move block[3, 2, 1] to the right if block[0] is empty
        if bytes & 0xFF == 0 {
            bytes >>= 8;
        }

        // Early return if block[0] is empty (meaning all blocks were zero)
        if bytes & 0xFF == 0 {
            return bytes;
        }
        // Merge block[0] and block[1] if possible
        if (bytes ^ (bytes >> 8)) & 0xFF == 0 {
            bytes = (bytes >> 8) + 1;
        }

        // Early return if block[1] is empty (meaning the rest of blocks are zero)
        if bytes & 0xFF00 == 0 {
            return bytes;
        }
        // Merge block[1] and block[2] if possible
        if (bytes ^ (bytes >> 8)) & 0xFF00 == 0 {
            // PERF: use fast bit merge
            bytes = (bytes >> 8) & !0xFF | (bytes & 0xFF);
            bytes += 1 << 8;
        }

        // Early return if block[2] is empty (meaning the rest of blocks are zero)
        if bytes & 0xFF0000 == 0 {
            return bytes;
        }
        // Merge block[2] and block[3] if possible
        if (bytes ^ (bytes >> 8)) & 0xFF0000 == 0 {
            // PERF: use fast bit merge
            bytes = (bytes >> 8) & !0xFFFF | (bytes & 0xFFFF);
            bytes += 1 << 16;
        }

        bytes
    }

    #[inline]
    pub fn swipe_left(&mut self) -> SwipeResult {
        if let [a, b, c, d] = self.0[..] {
            let bytes = [a, b, c, d].map(|mb| mb.map(|n| n.get()).unwrap_or(0));
            let bytes = u32::from_le_bytes(bytes);
            let swiped_bytes = Self::swipe_4_fast(bytes);
            let row_4 = Row::<4>::from(swiped_bytes.to_le_bytes());

            assert_eq!(SIZE, 4);
            // SAFETY: SIZE==4 so the pointer is valid and dereferencing is fine
            *self = unsafe { *(&row_4 as *const Row<4> as *const Row<SIZE>) };

            return if bytes == swiped_bytes {
                SwipeResult::UnChanged
            } else {
                // TODO: add score computation for the fast case
                SwipeResult::Changed { score: 0 }
            };
        }

        let mut last_pos = 0;
        let mut result = SwipeResult::UnChanged;
        let mut cells = &mut self.0;

        for i in 1..cells.len() {
            match cells[i] {
                None => {}

                // merge
                Some(block) if cells[last_pos] == Some(block) => {
                    let merged_block = block.get().wrapping_add(1);

                    cells[last_pos] = Block::new(merged_block);
                    cells[i] = None;

                    if let Some(merged_block) = cells[last_pos] {
                        result.handle_merge(merged_block);
                    }

                    last_pos += 1;
                }

                // move
                Some(_block) => {
                    if cells[last_pos].is_some() {
                        last_pos += 1;
                    }

                    cells.swap(last_pos, i);

                    if last_pos != i {
                        result.handle_move()
                    }
                }
            }
        }

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
pub fn swipe_left<const SIZE: usize>(cells: &mut [u8; SIZE]) -> bool {
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
        let test_cases = [
            ([0, 0, 0, 0], [0, 0, 0, 0]),
            ([0, 0, 0, 1], [1, 0, 0, 0]),
            ([0, 0, 1, 0], [1, 0, 0, 0]),
            ([0, 0, 1, 1], [2, 0, 0, 0]),
            ([1, 0, 1, 1], [2, 1, 0, 0]),
            ([1, 1, 1, 0], [2, 1, 0, 0]),
            ([0, 2, 1, 2], [2, 1, 2, 0]),
            ([1, 3, 2, 4], [1, 3, 2, 4]),
            ([1, 2, 2, 3], [1, 3, 3, 0]),
            ([1, 0, 1, 2], [2, 2, 0, 0]),
            ([1, 1, 1, 1], [2, 2, 0, 0]),
            ([2, 1, 0, 1], [2, 2, 0, 0]),
        ];

        for (inp, out) in test_cases.into_iter() {
            let inp = Row::from(inp);
            let out = Row::from(out);
            let mut row = inp;
            row.swipe_left();
            assert_eq!(row, out, "input: {inp:?}");
        }

        let mut row = Row::from([]);
        row.swipe_left();
        assert_eq!(row, Row::from([]));

        let mut row = Row::from([1]);
        row.swipe_left();
        assert_eq!(row, Row::from([1]));
    }
}
