use rust_2048_solver::{
    bots::mean_max::{Decision, MeanMax, SearchConstraint},
    game::twenty_forty_eight::TwentyFortyEight,
    utils,
};
use std::{
    io::Write,
    time::{Duration, Instant},
};

fn main() {
    // TODO: Add command line arguments for deadline.

    // show_map(heuristic::get_lookup());

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .parse_default_env()
        .init();

    let measure_performance_mode = false;
    if measure_performance_mode {
        return measure_performance();
    }

    let mut ai = MeanMax::new();

    ai.logger.log_search_results = true;
    ai.logger.log_cache_info = false;
    ai.logger.log_deadline_miss = true;
    ai.logger.clear_screen = true;
    ai.logger.print_size_of_critical_structs = false;

    let auto_adjust_search_time = false;
    let base_search_time = Duration::from_secs_f64(0.02);

    let mut search_time_multiplier = 1;

    if ai.logger.clear_screen {
        rust_2048_solver::init_screen();
    }

    let mut game = TwentyFortyEight::<4, 4>::new();
    println!("{}", game.state);
    loop {
        let search_duration = search_time_multiplier * base_search_time;
        let deadline = Instant::now() + search_duration;

        let search_constraint = SearchConstraint::new().with_deadline(deadline);

        let decision = ai.decide_until(&game, search_constraint);

        // utils::print_lookup(&ai);
        // utils::print_model(&ai.model);
        utils::show_fill_percent(&ai);

        let act = match decision {
            Decision::Act(act) => act,
            Decision::Resign => {
                log::info!("The agent resigned!");
                break;
            }
        };

        log::info!("Action: {action}", action = act.action);

        let reward = game.step(act.action);
        println!("{}", game.state);

        if reward.is_none() {
            // The game has ended
            break;
        }

        #[allow(clippy::match_overlapping_arm)]
        if auto_adjust_search_time {
            search_time_multiplier = match act.eval.value as u32 {
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

    if ai.logger.clear_screen {
        rust_2048_solver::end_screen();
    }

    println!("{}", game.state);
    println!("Game Over!");
    // utils::print_lookup(&ai);
}

fn measure_performance() {
    const N_SAMPLES: i32 = 100;

    log::info!("Collecting {N_SAMPLES} samples");
    if log::log_enabled!(log::Level::Info) {
        let total_score: f32 = (0..N_SAMPLES)
            .map(|i| {
                print!(
                    "\rCollecting sample {sample_number}/{N_SAMPLES}",
                    sample_number = i + 1
                );

                std::io::stdout().flush().expect("failed to flush stdout");

                rust_2048_solver::measure_performance()
            })
            .sum();

        // Go to the next line for the log
        println!();

        let average_score = total_score / N_SAMPLES as f32;
        log::info!("Average performance over {N_SAMPLES} runs: {average_score}")
    }
}
