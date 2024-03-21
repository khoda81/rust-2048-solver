use criterion::criterion_main;

mod board;
mod mean_max_search;
// TODO: add benchmarking for heuristic

criterion_main!(board::board, mean_max_search::mean_max_search);
