use rust_2048_solver::{bots::mean_max, game, utils};
use std::time::{Duration, Instant};

fn main() {
    // TODO: Add command line arguments for deadline.

    // show_map(heuristic::get_lookup());

    let mut game = game::GameState::<4, 4>::create();
    let mut ai = mean_max::MeanMax::new();

    ai.logger.log_search_results = true;
    ai.logger.log_cache_info = false;
    ai.logger.clear_screen = true;
    ai.logger.show_size_of_critical_structs = false;

    let auto_adjust_search_time = true;
    let base_search_time = Duration::from_secs_f64(0.02);

    let mut search_time_multiplier = 1;

    println!("{}", game.state);
    loop {
        let search_duration = search_time_multiplier * base_search_time;
        let deadline = Instant::now() + search_duration;

        let search_constraint = mean_max::SearchConstraint::new().with_deadline(deadline);

        let decision = ai.decide_until(&game, search_constraint);

        // utils::print_lookup(&ai);
        // utils::print_model(&ai.model);
        utils::show_fill_percent(&ai);

        let Some(decision) = decision else {
            println!("The agent resigned!");
            break;
        };

        println!("Action: {action}", action = decision.action);

        let reward = game.full_step(decision.action);
        println!("{}", game.state);

        if reward.is_none() {
            break;
        }

        #[allow(clippy::match_overlapping_arm)]
        if auto_adjust_search_time {
            search_time_multiplier = match decision.eval.value as u32 {
                ..=20 => 200,
                ..=50 => 100,
                ..=100 => 50,
                ..=200 => 15,
                ..=500 => 5,
                ..=1000 => 2,
                _ => 1,
            };
        }
    }

    println!("Game Over!");
    // utils::print_lookup(&ai);
}
