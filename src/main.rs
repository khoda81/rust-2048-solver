#![allow(unused_imports)]

use itertools::Itertools;

use std::{
    collections::hash_map::Entry,
    fmt::{Debug, Display, Write},
    mem,
    time::{Duration, Instant},
};

use rust_2048_solver::{
    bots::{
        self,
        dfs::{SearchConstraint, SearchResult},
        model::weighted::{self, Weighted},
    },
    game, utils,
};

fn main() {
    // show_map(heuristic::get_lookup());

    let mut game = game::GameOf2048::<4, 4>::create();
    let mut ai = bots::dfs::MeanMax::new();

    ai.logger.print_search_results = true;
    ai.logger.print_hit_info = false;
    let mut search_duration = Duration::from_secs_f64(0.1);

    loop {
        println!("{}", game.board);

        let deadline = Instant::now() + search_duration;

        #[allow(clippy::needless_update)]
        let search_constraint = SearchConstraint {
            deadline: Some(deadline),
            // max_depth: 3,
            // Set the remaining values to defaults
            ..Default::default()
        };

        let SearchResult { eval, action } = ai.search_until(&game.board, search_constraint);

        search_duration = match eval.value as u32 {
            0..=20 => Duration::from_secs_f64(20.0),
            21..=50 => Duration::from_secs_f64(10.0),
            51..=100 => Duration::from_secs_f64(5.0),
            101..=200 => Duration::from_secs_f64(1.5),
            201..=500 => Duration::from_secs_f64(0.5),
            501..=1000 => Duration::from_secs_f64(0.2),
            _ => Duration::from_secs_f64(0.1),
        };

        // search_duration = Duration::from_secs_f64(60.0);

        // utils::print_lookup(&ai);
        // utils::print_model(&ai.model);

        println!("{action}");
        if !game.step(action) {
            break;
        }
    }

    println!("{}", game.board);
    utils::print_lookup(&ai);
}
