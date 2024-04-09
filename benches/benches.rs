use criterion::criterion_main;

mod board;
mod conversions;
mod mean_max_search;
// TODO: Add benchmarking for heuristic.

criterion_main!(
    board::board,
    mean_max_search::mean_max_search,
    conversions::bench_conversions,
);
