#![feature(test)]

#[cfg(test)]
mod benches {
    extern crate test;

    use rust_2048_solver::{
        board::{Board, Direction},
        bots::dfs::{MeanMax, SearchConstraint},
    };

    use test::{black_box, Bencher};

    #[bench]
    fn bench_search(b: &mut Bencher) {
        let mut ai = MeanMax::new();

        let mut board: Board<4, 4>;

        board = [
            // BOARD
            [3, 3, 1, 1],
            [1, 0, 5, 0],
            [0, 2, 7, 4],
            [6, 1, 6, 8],
        ]
        .into();
        board.swipe(Direction::Down);

        println!("{board}");
        b.iter(|| {
            ai.player_cache.clear();

            let search_constrain = SearchConstraint {
                max_depth: 4,
                ..Default::default()
            };

            let result = ai.search_until(&board, search_constrain);

            black_box(result);
            // show_fill_percent(&ai);
        });
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
