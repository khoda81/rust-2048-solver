#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use rust_2048_solver::{
        board::{Board, Direction},
        bots::dfs::DFS,
    };
    use test::{black_box, Bencher};

    #[bench]
    fn bench_search(b: &mut Bencher) {
        let mut ai = DFS::new();

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

        ai.deadline = Instant::now() + Duration::from_secs(1_000_000_000);

        println!("{board}");
        b.iter(|| {
            ai.player_cache.clear();
            let result = ai
                .evaluate_for_player(&board, 5)
                .expect("we should not reach the deadline");

            black_box(result);
            // show_fill_percent(&ai);
        });
    }

    fn show_fill_percent(ai: &DFS<4, 4>) {
        let capacity = ai.player_cache.cap().get();
        let filled = ai.player_cache.len();
        let fill_percent = (filled * 100) as f64 / capacity as f64;
        println!("fill = {fill_percent:.2?}%",);
    }
}
