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
    // show_map(heuristic::get_lookup());

    let mut game = Game::<4, 4>::create();
    let mut ai = DFS::new();

    loop {
        println!("{}", game.board);

        let timeout = Duration::from_secs_f64(1.0);
        let deadline = Instant::now() + timeout;

        let action = ai.act(&game.board, deadline);

        let miss = deadline.elapsed();
        if !miss.is_zero() {
            println!("missed: {miss:?}");
        }

        // print_lookup(&ai);

        println!("{action}");
        if !game.step(action) {
            break;
        }
    }

    println!("{}", game.board);
    print_lookup(&ai);
}

fn print_lookup(ai: &DFS<4, 4>) {
    let mut new_lookup = heuristic::get_lookup().clone();

    for (key, eval) in ai.model.evaluation_memory.iter() {
        new_lookup.insert(*key, eval.value.mean());
    }

    show_map(&new_lookup);
}

fn show_map<V: std::fmt::Debug>(map: &HashMap<heuristic::PreprocessedBoard, V>) {
    for (key, value) in map
        .iter()
        .sorted_by_key(|(&(empty, max), _eval)| (max, empty))
    {
        println!("map.insert({key:2?}, {value:?});");
        // println!("data[{key:?}] = {value}");
    }
}
