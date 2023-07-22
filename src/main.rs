#![allow(unused_imports)]

use rust_2048_solver::board::Board;
use std::{
    collections::{hash_map::Entry, HashMap},
    time::{Duration, Instant},
};

use rust_2048_solver::{
    board::Direction,
    game::{Game, DFS},
};

pub fn main1() {
    let mut game = Game::<4, 4>::create();
    game.board = [
        // BOARD
        [3, 3, 1, 1],
        [1, 9, 5, 0],
        [10, 2, 7, 4],
        [6, 1, 6, 8],
    ]
    .into();

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

        let timeout = Duration::from_secs_f64(0.5);
        let deadline = Instant::now() + timeout;

        let result = ai.evaluate_until(&game.board, deadline);

        let miss = deadline.elapsed();
        if !miss.is_zero() {
            println!("missed: {miss:?}");
        }

        println!("{result:.2?}");
        if game.step(result.action) {
            break;
        }
    }

    println!("{:?}", game);
}

pub fn main() {
    let board = [
        // BOARD
        [0, 1, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 0],
    ]
    .into();

    let mut ai = DFS::new();

    let start = Instant::now();
    let deadline = start + Duration::from_secs_f64(2.5);

    ai.cached_evaluate_by_depth(&board, 4, deadline);
    let result = ai.cached_evaluate_by_depth(&board, 5, deadline);

    println!("{:?}", start.elapsed());
    println!("{result:?}");
}
