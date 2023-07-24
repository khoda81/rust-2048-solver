#![allow(unused_imports)]

use rust_2048_solver::board::Board;
use std::{
    collections::{hash_map::Entry, HashMap},
    time::{Duration, Instant},
};

use rust_2048_solver::{board::Direction, bots::dfs::DFS, game::Game};

pub fn play() {
    let mut game = Game::<4, 4>::create();
    // game.board = [
    //     // BOARD
    //     [3, 3, 1, 1],
    //     [1, 9, 5, 0],
    //     [10, 2, 7, 4],
    //     [6, 1, 6, 8],
    // ]
    // .into();

    let mut ai = DFS::new();

    loop {
        println!("{}", game.board);

        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
        // input = input.trim().to_lowercase();

        // let action = match input.as_str() {
        //     "w" => Direction::Up,
        //     "a" => Direction::Left,
        //     "s" => Direction::Down,
        //     "d" => Direction::Right,
        //     "q" => break,
        //     _ => continue,
        // };

        // let action: Direction = rand::random();

        let timeout = Duration::from_secs_f64(0.2);
        let deadline = Instant::now() + timeout;

        let action = ai.act(&game.board, deadline);

        let miss = deadline.elapsed();
        if !miss.is_zero() {
            println!("missed: {miss:?}");
        }

        println!("{action:?}");
        if game.step(action) {
            break;
        }
    }

    println!("{}", game.board);
}

pub fn benchmark() {
    let mut ai = DFS::new();

    let mut board;
    board = [
        // BOARD
        [3, 3, 7, 1],
        [0, 9, 5, 0],
        [10, 0, 7, 4],
        [6, 1, 6, 8],
    ]
    .into();

    // fill cache for a more accurate benchmark
    ai.act(&board, Instant::now() + Duration::from_secs_f64(1.9));

    let fill = (ai.evaluation_cache.len() * 100) as f64 / ai.evaluation_cache.cap().get() as f64;
    println!("fill = {fill:?}%",);

    board = [
        // BOARD
        [3, 3, 1, 1],
        [1, 9, 5, 0],
        [10, 2, 7, 4],
        [6, 1, 6, 8],
    ]
    .into();

    let start = Instant::now();
    let deadline = start + Duration::from_secs_f64(1.0);

    let result = ai.evaluate_by_depth(&board, 8, deadline);

    println!("{:?}", start.elapsed());
    println!("{result:?}");

    let fill = (ai.evaluation_cache.len() * 100) as f64 / ai.evaluation_cache.cap().get() as f64;
    println!("fill = {fill:?}%",);
}

fn main() {
    benchmark();
    // play();
}
