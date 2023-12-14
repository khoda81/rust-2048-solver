#![allow(unused_imports)]

use itertools::Itertools;

use std::{
    collections::hash_map::Entry,
    fmt::{Debug, Display, Write},
    mem,
    sync::{mpsc, Mutex},
    time::{Duration, Instant},
};

use rust_2048_solver::{
    bots::{
        self,
        mean_max::{Bound, EvaluatedAction, SearchConstraint},
        model::weighted::{self, Weighted},
    },
    game, utils,
};

fn main() {
    // show_map(heuristic::get_lookup());

    let mut game = game::GameOf2048::<4, 4>::create();
    let mut ai = bots::mean_max::MeanMax::new();

    game.state = [
        [0, 0, 0, 0],
        [0, 1, 13, 14],
        [15, 16, 17, 18],
        [19, 20, 21, 22],
    ]
    .into();

    ai.logger.log_search_results = true;
    // ai.logger.log_cache_info = true;
    // ai.logger.clear_screen = true;
    ai.logger.show_size_of_critical_structs = true;

    let mut search_duration = Duration::from_secs_f64(200.1);

    loop {
        println!("{}", game.state);

        let deadline = Instant::now() + search_duration;

        #[allow(clippy::needless_update)]
        let search_constraint = SearchConstraint {
            deadline: Some(deadline),
            // max_depth: Bound::new(3),

            // Set the remaining values to defaults
            ..Default::default()
        };

        let EvaluatedAction { eval, action } = ai
            .decide_until(&game.state, search_constraint)
            .expect("the game is not over, the ai returned None");

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
        utils::show_fill_percent(&ai);

        println!("Action: {action}");
        if !game.step(action) {
            break;
        }
    }

    println!("{}\n", game.state);
    println!("Game Over!");

    // utils::print_lookup(&ai);
}
