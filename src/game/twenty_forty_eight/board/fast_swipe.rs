use std::num::NonZeroU8;

pub fn swipe_left_4_fast(mut bytes: u32) -> u32 {
    // Early return if all are zeros
    // if bytes == 0 {
    //     return bytes;
    // }

    // println!("Input: 0x{bytes:08x} ({:?})", bytes.to_be_bytes());
    // Move block[3] to the right if block[2] is empty
    if bytes & 0xFF0000 == 0 {
        bytes = (bytes >> 8) & !0xFFFF | bytes & 0xFFFF;
    }

    // println!("shift1: 0x{bytes:08x} ({:?})", bytes.to_be_bytes());
    // Move block[3, 2] to the right if block[1] is empty
    if bytes & 0xFF00 == 0 {
        bytes = (bytes >> 8) & !0xFF | bytes & 0xFF;
    }

    // println!("shift2: 0x{bytes:08x}({:?})", bytes.to_be_bytes());
    // Move block[3, 2, 1] to the right if block[0] is empty
    if bytes & 0xFF == 0 {
        bytes >>= 8;
    }

    // println!("shift3: 0x{bytes:08x} ({:?})", bytes.to_be_bytes());
    // Early return if block[0] is empty (meaning all blocks were zero)
    if bytes & 0xFF == 0 {
        return bytes;
    }
    // Merge block[0] and block[1] if possible
    if (bytes ^ (bytes >> 8)) & 0xFF == 0 {
        bytes = (bytes >> 8) + 1;
    }
    // // Undo merge if block[0] is empty (meaning all blocks were zero)
    // if bytes & 0xFF == 1 {
    //     bytes -= 1;
    // }

    // println!("merge1: 0x{bytes:08x} ({:?})", bytes.to_be_bytes());
    // Early return if block[1] is empty (meaning the rest of blocks are zero)
    if bytes & 0xFF00 == 0 {
        return bytes;
    }
    // Merge block[1] and block[2] if possible
    if (bytes ^ (bytes >> 8)) & 0xFF00 == 0 {
        bytes = (bytes >> 8) & !0xFF | (bytes & 0xFF);
        bytes += 1 << 8;
    }
    // // Undo Merge if block[1] is empty (meaning the rest of blocks are zero)
    // if bytes & 0xFFFF00 == (1 << 8) {
    //     bytes -= 1 << 8;
    // }

    // Early return if block[1] is empty (meaning the rest of blocks are zero)
    if bytes & 0xFF0000 == 0 {
        return bytes;
    }
    // println!("merge2: 0x{bytes:08x} ({:?})", bytes.to_be_bytes());
    // Merge block[2] and block[3] if possible
    if (bytes ^ (bytes >> 8)) & 0xFF0000 == 0 {
        bytes = (bytes >> 8) & !0xFFFF | (bytes & 0xFFFF);
        bytes += 1 << 16;
    }
    // // Undo Merge if block[2] is empty (meaning the rest of blocks are zero)
    // if bytes & 0xFF0000 == (1 << 16) {
    //     bytes -= 1 << 16;
    // }

    // println!("merge3: 0x{bytes:08x} ({:?})", bytes.to_be_bytes());

    bytes
}

type Block = NonZeroU8;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum SwipeResult {
    #[default]
    Unchanged,
    Changed {
        score: u32,
    },
}

impl SwipeResult {
    fn handle_merge(&mut self, merged_block: Block) {
        let new_score = 2u32.pow(merged_block.get().into());
        match self {
            SwipeResult::Unchanged => *self = SwipeResult::Changed { score: new_score },
            SwipeResult::Changed { score } => *score += new_score,
        }
    }

    fn handle_move(&mut self) {
        match self {
            SwipeResult::Unchanged => *self = SwipeResult::Changed { score: 0 },
            SwipeResult::Changed { .. } => {}
        }
    }

    /// Returns `true` if the swipe result is [`UnChanged`].
    ///
    /// [`UnChanged`]: SwipeResult::UnChanged
    #[must_use]
    fn is_unchanged(&self) -> bool {
        matches!(self, Self::Unchanged)
    }
}

pub fn swipe_left<const SIZE: usize>(cells: &mut [u8; SIZE]) -> bool {
    // TODO: Try this fast implementation for swipe_right.

    // if let [a, b, c, d] = cells[..] {
    //     let swiped_cells = swipe_left_4_fast(u32::from_le_bytes([a, b, c, d])).to_le_bytes();
    //
    //     let mut new_cells = [0; SIZE];
    //     new_cells
    //         .iter_mut()
    //         .zip(swiped_cells)
    //         .for_each(|(arr, cell)| *arr = cell);
    //
    //     let changed = cells != &new_cells;
    //     *cells = new_cells;
    //
    //     return changed;
    // }

    let mut last_pos = 0;
    let mut result = SwipeResult::Unchanged;
    let mut blocks = cells.map(Block::new);
    for i in 1..blocks.len() {
        match blocks[i] {
            None => {}

            // merge
            Some(block) if blocks[last_pos] == Some(block) => {
                let merged_block = block.get().wrapping_add(1);

                blocks[last_pos] = Block::new(merged_block);
                blocks[i] = None;

                if let Some(merged_block) = blocks[last_pos] {
                    result.handle_merge(merged_block);
                }

                last_pos += 1;
            }

            // move
            Some(_) => {
                if blocks[last_pos].is_some() {
                    last_pos += 1;
                }

                blocks.swap(last_pos, i);

                if last_pos != i {
                    result.handle_move()
                }
            }
        }
    }

    let new_cells = blocks.map(|block| block.map(NonZeroU8::get).unwrap_or(0));
    // if let [a, b, c, d] = cells[..] {
    //     let swiped_cells = swipe_left_4_fast(u32::from_le_bytes([a, b, c, d])).to_le_bytes();
    //     let mut swiped_cells_arr = [0; SIZE];
    //     swiped_cells_arr
    //         .iter_mut()
    //         .zip(swiped_cells)
    //         .for_each(|(arr, cell)| *arr = cell);
    //
    //     assert_eq!(new_cells, swiped_cells_arr, "Inp: {cells:?}");
    //     let is_unchanged = cells[..] == swiped_cells;
    //     assert_eq!(
    //         !result.is_unchanged(),
    //         !is_unchanged,
    //         "Inp: {cells:?}, Out: {new_cells:?}"
    //     );
    //     *cells = swiped_cells_arr;
    // } else {
    // }

    *cells = new_cells;
    !result.is_unchanged()
}

pub fn swipe_right<const SIZE: usize>(cells: &mut [u8; SIZE]) -> bool {
    // PERF: Reverse the implementation of swipe_left for this.
    cells.reverse();
    let swiped = swipe_left(cells);
    cells.reverse();
    swiped
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CASES: &[([u8; 4], [u8; 4])] = &[
        ([0, 0, 0, 0], [0, 0, 0, 0]),
        ([0, 0, 0, 1], [1, 0, 0, 0]),
        ([0, 0, 1, 0], [1, 0, 0, 0]),
        ([0, 1, 0, 0], [1, 0, 0, 0]),
        ([0, 0, 1, 1], [2, 0, 0, 0]),
        ([0, 2, 1, 2], [2, 1, 2, 0]),
        ([0, 1, 2, 1], [1, 2, 1, 0]),
        ([0, 2, 2, 2], [3, 2, 0, 0]),
        ([1, 0, 0, 0], [1, 0, 0, 0]),
        ([1, 0, 1, 1], [2, 1, 0, 0]),
        ([1, 0, 1, 2], [2, 2, 0, 0]),
        ([1, 1, 1, 0], [2, 1, 0, 0]),
        ([1, 1, 1, 1], [2, 2, 0, 0]),
        ([1, 2, 2, 3], [1, 3, 3, 0]),
        ([1, 3, 2, 4], [1, 3, 2, 4]),
        ([2, 1, 0, 1], [2, 2, 0, 0]),
        ([2, 2, 2, 2], [3, 3, 0, 0]),
        ([2, 3, 2, 2], [2, 3, 3, 0]),
    ];

    fn test_for<const SIZE: usize>(
        f: impl FnOnce(&mut [u8; SIZE]) -> bool,
        inp: [u8; SIZE],
        expected_out: [u8; SIZE],
    ) {
        let mut row = inp;
        let changed = f(&mut row);
        assert_eq!(row, expected_out, "input: {inp:?}");
        assert_eq!(changed, inp != expected_out, "input: {inp:?}");
    }

    #[test]
    fn test_swipe_left() {
        for (inp, expected_out) in TEST_CASES.iter().copied() {
            test_for(swipe_left, inp, expected_out);
        }

        test_for(swipe_left, [], []);
        test_for(swipe_left, [1], [1]);
    }

    #[test]
    fn test_swipe_right() {
        for (inp, expected_out) in TEST_CASES.iter().copied().map(|(mut inp, mut out)| {
            inp.reverse();
            out.reverse();
            (inp, out)
        }) {
            test_for(swipe_right, inp, expected_out);
        }

        test_for(swipe_right, [], []);
        test_for(swipe_right, [1], [1]);
    }
}
