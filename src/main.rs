#![allow(unused_imports)]

use itertools::Itertools;
use std::{
    collections::{hash_map::Entry, HashMap},
    time::{Duration, Instant},
};

use rust_2048_solver::{
    bots::{self, heuristic},
    game,
};

fn main() {
    // show_map(heuristic::get_lookup());

    let mut game = game::Game::<4, 4>::create();
    let mut ai = bots::dfs::MeanMax::new();

    loop {
        println!("{}", game.board);

        let timeout = Duration::from_secs_f64(0.2);
        let deadline = Instant::now() + timeout;

        let action = ai.act(&game.board, deadline);

        let miss = Instant::now().saturating_duration_since(deadline);
        if !miss.is_zero() {
            println!("Deadline missed by {miss:?}");
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

fn print_lookup<const ROWS: usize, const COLS: usize>(ai: &bots::dfs::MeanMax<ROWS, COLS>) {
    let mut new_lookup = heuristic::get_lookup().clone();

    for (key, eval) in ai.model.memory.iter() {
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
