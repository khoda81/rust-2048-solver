#![allow(unused_imports)]

use itertools::Itertools;
use std::{
    collections::{hash_map::Entry, HashMap},
    time::{Duration, Instant},
};

use rust_2048_solver::{
    bots::{
        self,
        dfs::SearchConstraint,
        heuristic,
        model::{weighted_avg, WeightedAvgModel},
    },
    game,
};

fn main() {
    // show_map(heuristic::get_lookup());

    let mut game = game::Game::<4, 4>::create();
    let mut ai = bots::dfs::MeanMax::new();

    let mut deadline_miss_model = weighted_avg::WeightedAvg::new();

    loop {
        println!("{}", game.board);

        let timeout = Duration::from_secs_f64(0.2);
        let deadline = Instant::now() + timeout;

        #[allow(clippy::needless_update)]
        let search_constraint = SearchConstraint {
            deadline: Some(deadline),
            // depth: Some(3),
            ..Default::default()
        };

        let action = ai.act(&game.board, search_constraint);

        let now = Instant::now();
        let miss = now.checked_duration_since(deadline);
        let miss_seconds = miss
            .map(|miss| miss.as_secs_f32())
            .unwrap_or(-deadline.duration_since(now).as_secs_f32());

        deadline_miss_model.add_sample(miss_seconds, 1.0);

        if let Some(miss) = miss {
            println!("Deadline missed by {miss:?}");
        }

        if let Ok(avg_miss) = Duration::try_from_secs_f32(deadline_miss_model.mean()) {
            println!("Avg miss: {avg_miss:?}");
        } else if let Ok(time_loss) = Duration::try_from_secs_f32(-deadline_miss_model.mean()) {
            println!("Avg time loss: {:?}", time_loss);
        }

        // print_lookup(&ai);

        println!("{action}");
        if !game.step(action) {
            break;
        }
    }

    println!("{}", game.board);
    // print_lookup(&ai);
}

pub fn print_lookup<const ROWS: usize, const COLS: usize>(ai: &bots::dfs::MeanMax<ROWS, COLS>) {
    let mut new_lookup = heuristic::get_lookup().clone();

    for (key, eval) in ai.model.memory.iter() {
        new_lookup.insert(*key, eval.value.mean());
    }

    show_map(&new_lookup);
}

pub fn show_map<V: std::fmt::Debug>(map: &HashMap<heuristic::PreprocessedBoard, V>) {
    for (key, value) in map
        .iter()
        .sorted_by_key(|(&(empty, max), _eval)| (max, empty))
    {
        println!("map.insert({key:2?}, {value:?});");
        // println!("data[{key:?}] = {value}");
    }
}
