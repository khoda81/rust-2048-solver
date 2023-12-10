#![feature(test)]

#[cfg(test)]
mod board {
    extern crate test;

    use std::collections::HashSet;

    use rand::seq::SliceRandom;
    use rust_2048_solver::board::{Board, Direction};

    use test::{black_box, Bencher};

    type State = Board<4, 4>;

    #[bench]
    fn bench_hash(b: &mut Bencher) {
        let boards = generate_boards(1000);

        println!("Hashing {} states", boards.len());
        b.iter(|| {
            let set = HashSet::<State>::from_iter(boards.clone());
            black_box(set);
        });
    }

    fn generate_boards(count: usize) -> Vec<Board<4, 4>> {
        let starting_board: State = [
            // BOARD
            [3, 3, 1, 1],
            [1, 0, 5, 0],
            [0, 2, 7, 4],
            [6, 1, 6, 8],
        ]
        .into();

        let mut boards = Vec::from([starting_board]);
        let mut rng = rand::thread_rng();
        while boards.len() < count {
            let board = *boards.choose(&mut rng).unwrap();
            let iter = board
                .iter_transitions()
                .map(|transition| transition.new_state)
                .flat_map(|new_board| new_board.spawns())
                .map(|(new_board, _)| new_board);

            boards.extend(iter);
        }

        boards.truncate(count);

        boards
    }

    #[bench]
    fn bench_board_swipe(b: &mut Bencher) {
        let mut board = Board::<4, 4>::new();

        b.iter(|| {
            (0..10_000).for_each(|_| {
                board.swipe(Direction::Up);
                board.swipe(Direction::Right);
                board.swipe(Direction::Down);
                board.swipe(Direction::Left);

                if board.is_lost() {
                    board = Board::new();
                }
            })
        });
    }
}
