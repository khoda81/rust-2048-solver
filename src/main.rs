#![allow(unused_imports)]

use itertools::Itertools;

use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{Debug, Display, Write},
    mem,
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Signed<T> {
    Positive(T),
    Negative(T),
}

impl<T: Display> Display for Signed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            Signed::Positive(inner) => inner,
            Signed::Negative(inner) => {
                f.write_char('-')?;
                inner
            }
        };

        inner.fmt(f)
    }
}

impl<T: Debug> Debug for Signed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = match self {
            Signed::Positive(inner) => inner,
            Signed::Negative(inner) => {
                f.write_char('-')?;
                inner
            }
        };

        inner.fmt(f)
    }
}

fn main() {
    // show_map(heuristic::get_lookup());

    let mut game = game::Game::<4, 4>::create();
    let mut ai = bots::dfs::MeanMax::new();

    ai.logger.print_search_results = true;

    loop {
        println!("{}", game.board);

        let timeout = Duration::from_secs_f64(0.2);
        let deadline = Instant::now() + timeout;

        #[allow(clippy::needless_update)]
        let search_constraint = SearchConstraint {
            deadline: Some(deadline),
            // max_depth: 3,
            // Set the ramaining values to defaults
            ..Default::default()
        };

        let action = ai.act(&game.board, search_constraint);

        // TODO: Move search info logic to the logger
        let now = Instant::now();
        let miss_seconds = if deadline <= now {
            (now - deadline).as_secs_f64()
        } else {
            -(deadline - now).as_secs_f64()
        };

        ai.logger.deadline_miss_model.add_sample(miss_seconds, 1.0);

        // println!("Hit chance per depth:");
        // println!("{:.2}", ai.logger.cache_hit_chance_model);

        // println!("Hit depth per depth:");
        // println!("{:.2}", ai.logger.cache_hit_depth_model);

        println!("Deadline missed by {:?}", get_signed_duration(miss_seconds));
        let avg_miss = get_signed_duration(ai.logger.deadline_miss_model.mean());
        println!("Avg miss: {avg_miss:?}");

        // print_lookup(&ai);

        println!("{action}");
        if !game.step(action) {
            break;
        }
    }

    println!("{}", game.board);
    // print_lookup(&ai);
}

pub fn get_signed_duration(seconds: f64) -> Signed<Duration> {
    let abs_duration = Duration::from_secs_f64(seconds.abs());
    if seconds.is_sign_positive() {
        Signed::Positive(abs_duration)
    } else {
        Signed::Negative(abs_duration)
    }
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
