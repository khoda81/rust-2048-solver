use rust_2048_solver::board::Board;
use std::time::{Duration, Instant};

use rust_2048_solver::{
    board::Direction,
    game::{Game, DFS},
};

fn main() {
    let mut game = Game::<4, 4>::create();
    let mut ai = DFS::new();

    loop {
        println!("{:?}", game);
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
        // input = input.trim().to_lowercase();

        // let direction = match input.as_str() {
        //     "w" => Direction::Up,
        //     "a" => Direction::Left,
        //     "s" => Direction::Down,
        //     "d" => Direction::Right,
        //     "q" => break,
        //     _ => continue,
        // };

        // let direction: Direction = rand::random();
        // println!("{:?}", direction);

        let timeout = Duration::from_secs_f64(0.01);
        let deadline = Instant::now() + timeout;
        let result = ai.evaluate_until(&game.board, deadline);
        // let result = ai.get(&game.board, 3, deadline);
        // println!("{result:?}");
        if let Some(miss) = Instant::now().checked_duration_since(deadline) {
            println!("missed: {miss:?}");
        }

        let (_depth, _expected, direction) = result;

        if game.step(direction) {
            break;
        }
    }

    println!("{:?}", game);
}

fn benchmark() -> Duration {
    let board = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]].into();

    let mut ai = DFS::new();
    let start = Instant::now();
    ai.evaluate_by_depth(&board, 4);
    start.elapsed()
}
