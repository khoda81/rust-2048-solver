use rust_2048_solver::{
    bots::mean_max::{
        searcher::{Decision, SearchConstraint},
        MeanMax,
    },
    game::{twenty_forty_eight::State, GameState, Outcome},
};
use std::io::Write;
use std::time::{Duration, Instant};

fn main() {
    // TODO: Add command line arguments.

    // show_map(heuristic::get_lookup());

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        //.filter_level(log::LevelFilter::Trace)
        .parse_default_env()
        .init();

    let measure_performance_mode = false;
    if measure_performance_mode {
        return measure_performance();
    }

    let mut ai = MeanMax::new();

    {
        let mut logger = ai.logger.lock().unwrap();
        logger.log_search_results = true;
        logger.log_deadline_miss = true;
        logger.clear_screen = true;
        logger.print_size_of_critical_structs = false;
    }

    let auto_adjust_search_time = false;
    let base_search_time = Duration::from_secs_f64(0.5);

    let mut search_time_multiplier = 1;

    if ai.logger.lock().unwrap().clear_screen {
        rust_2048_solver::init_screen();
    }

    let mut game = State::<4, 4>::new();
    println!("{}", game.cells);
    loop {
        let search_duration = search_time_multiplier * base_search_time;
        let deadline = Instant::now() + search_duration;

        let search_constraint = SearchConstraint::new().with_deadline(deadline);

        let decision = ai.decide_until(&game, search_constraint);

        // utils::print_lookup(&ai);
        // utils::print_model(&ai.model);
        // utils::show_fill_percent(&ai.evaluation_cache);

        let act = match decision {
            Decision::Act(act) => act,
            Decision::Resign => {
                log::info!("The agent resigned!");
                break;
            }
        };

        log::info!("Action: {action}", action = act.action);

        let (_reward, outcome) = game.outcome(act.action);
        game = outcome.collapse();
        println!("{}", game.cells);

        if game.is_terminal() {
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

    if ai.logger.lock().unwrap().clear_screen {
        rust_2048_solver::end_screen();
    }

    println!("{}", game.cells);
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
