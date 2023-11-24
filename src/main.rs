#![allow(unused_imports)]

use itertools::Itertools;
use rust_2048_solver::{
    board::Board,
    bots::heuristic::{self},
};
use std::{
    collections::{hash_map::Entry, HashMap},
    time::{Duration, Instant},
};

use rust_2048_solver::{board::Direction, bots::dfs::DFS, game::Game};

fn main() {
    show_map(heuristic::get_lookup());

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

        let timeout = Duration::from_secs_f64(10.0);
        let deadline = Instant::now() + timeout;

        let action = ai.act(&game.board, deadline);

        // let miss = deadline.elapsed();
        // if !miss.is_zero() {
        //     println!("missed: {miss:?}");
        // }
        let mut new_lookup = heuristic::get_lookup().clone();

        for (key, value) in ai.model.evaluation_memory.iter() {
            new_lookup.insert(*key, value.get_value());
        }

        show_map(&new_lookup);

        println!("{action}");
        if !game.step(action) {
            break;
        }
    }

    println!("{}", game.board);
    let mut new_lookup = heuristic::get_lookup().clone();

    for (key, value) in ai.model.evaluation_memory.into_iter() {
        new_lookup.insert(key, value.get_value());
    }

    show_map(&new_lookup);
}

fn show_map<K: std::cmp::Ord + std::fmt::Debug, V: std::fmt::Debug>(map: &HashMap<K, V>) {
    for (key, value) in map.iter().sorted_by_key(|(key, _eval)| *key) {
        println!("map.insert({key:?}, {value:?});");
        // println!("data[{key:?}] = {value}");
    }
}
