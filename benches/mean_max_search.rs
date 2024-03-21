use criterion::{criterion_group, Bencher, BenchmarkId, Criterion};
use rust_2048_solver::{
    bots::mean_max::{max_depth::MaxDepth as Bound, MeanMax, SearchConstraint},
    game,
};

fn run_search<const COLS: usize, const ROWS: usize>(
    b: &mut Bencher,
    input: &(game::Swipe2048<COLS, ROWS>, SearchConstraint),
) {
    let &(state, search_constraint) = input;

    b.iter_batched(
        MeanMax::new,
        |mut ai| ai.decide_until(&state, search_constraint),
        criterion::BatchSize::PerIteration,
    )
}

pub fn bench_search_depth(c: &mut Criterion) {
    let mut bench_search = |state: game::Swipe2048<4, 4>, constraint: SearchConstraint| {
        let parameter_display = format!("{constraint:?}");

        c.bench_with_input(
            BenchmarkId::new("search", parameter_display),
            &(state, constraint),
            run_search,
        );
    };

    #[rustfmt::skip]
    bench_search(
        [
            [3, 3, 1, 1],
            [1, 0, 5, 0],
            [0, 2, 7, 4],
            [6, 1, 6, 8],
        ]
        .into(),
        SearchConstraint::default().with_max_depth(Bound::new(3)),
    );

    #[rustfmt::skip]
    bench_search(
        [
            [ 3,  4,  6, 10],
            [ 2, 10,  3,  1],
            [ 0,  1,  7,  3],
            [ 0,  0,  2,  8],
        ]
        .into(),
        SearchConstraint::default().with_max_depth(Bound::new(4)),
    );

    #[rustfmt::skip]
    bench_search(
        [
            [ 0,  0,  0,  0],
            [11, 12, 13, 14],
            [15, 16, 17, 18],
            [19, 20, 21, 22],
        ]
        .into(),
        SearchConstraint::default(),
    );
}

criterion_group!(mean_max_search, bench_search_depth);
