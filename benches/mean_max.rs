#![feature(test)]

#[cfg(test)]
mod mean_max {
    extern crate test;

    use rust_2048_solver::{
        board::Board,
        bots::mean_max::{Bound, MeanMax, SearchConstraint},
    };

    use test::{black_box, Bencher};

    fn bench_board<const COLS: usize, const ROWS: usize>(
        b: &mut Bencher,
        board: Board<COLS, ROWS>,
        search_constrain: SearchConstraint,
    ) {
        let mut ai = MeanMax::new();

        b.iter(|| {
            ai.evaluation_cache.clear();

            let result = ai.search_until(&board, search_constrain);
            let _ = black_box(result);
        })
    }

    #[bench]
    fn bench_search_depth_3(b: &mut Bencher) {
        bench_board(
            b,
            [[3, 3, 1, 1], [1, 0, 5, 0], [0, 2, 7, 4], [6, 1, 6, 8]].into(),
            SearchConstraint {
                max_depth: Bound::new(3),
                ..Default::default()
            },
        )
    }

    #[bench]
    fn bench_search_depth_4(b: &mut Bencher) {
        bench_board(
            b,
            [[3, 4, 6, 10], [2, 10, 3, 1], [0, 1, 7, 3], [0, 0, 2, 8]].into(),
            SearchConstraint {
                max_depth: Bound::new(4),
                ..Default::default()
            },
        )
    }

    #[bench]
    fn bench_search_terminal(b: &mut Bencher) {
        bench_board(
            b,
            [
                [0, 0, 0, 0],
                [11, 12, 13, 14],
                [15, 16, 17, 18],
                [19, 20, 21, 22],
            ]
            .into(),
            SearchConstraint::default(),
        )
    }
}
