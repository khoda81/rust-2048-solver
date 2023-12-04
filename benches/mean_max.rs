#![feature(test)]

#[cfg(test)]
mod benches {
    extern crate test;

    use rust_2048_solver::{
        board::Board,
        bots::dfs::{MeanMax, SearchConstraint},
    };

    use test::{black_box, Bencher};

    #[bench]
    fn bench_search(b: &mut Bencher) {
        let mut ai = MeanMax::new();

        let board: Board<4, 4> = [
            // BOARD
            [3, 3, 1, 1],
            [1, 0, 5, 0],
            [0, 2, 7, 4],
            [6, 1, 6, 8],
        ]
        .into();

        println!("{board}");

        b.iter(|| {
            ai.player_cache.clear();

            let search_constrain = SearchConstraint {
                max_depth: 4,
                ..Default::default()
            };

            let result = ai.search_until(&board, search_constrain);
            black_box(result);
        });
    }
}
