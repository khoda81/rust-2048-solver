#![feature(slice_flatten)]
// TODO: Rename project to brickfish.

pub mod bots;
pub mod game;

pub mod utils;

pub fn init_screen() {
    use std::panic::take_hook;

    // Switch to alternate screen buffer
    print!("\x1b[?1049h");
    // Register the custom panic hook
    let current_hook = take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        end_screen();

        // Call the default panic hook
        current_hook(panic_info);
    }));

    // Handle signals
    let set_handler_result = ctrlc::set_handler(move || {
        end_screen();

        // Exit gracefully
        std::process::exit(0);
    });

    if let Err(err) = set_handler_result {
        log::error!("Failed to set ctrl_c hook: {err}")
    }
}

pub fn end_screen() {
    // Switch back to normal screen buffer
    println!("\x1b[?1049l");
}

pub fn measure_performance() -> f32 {
    use bots::mean_max::{Decision, MeanMax, SearchConstraint};
    use game::twenty_forty_eight::TwentyFortyEight;
    use std::time;

    let mut game = TwentyFortyEight::<4, 4>::new();
    let mut ai = MeanMax::new();
    let search_time = time::Duration::from_secs_f64(0.001);

    let mut deadline = time::Instant::now();
    let mut total_reward = 0.0;

    loop {
        deadline += search_time;

        let search_constraint = SearchConstraint::new().with_deadline(deadline);
        let decision = ai.decide_until(&game, search_constraint);
        let act = match decision {
            Decision::Act(act) => act,
            Decision::Resign => {
                // The agent resigned!
                break;
            }
        };

        if let Some(reward) = game.step(act.action) {
            total_reward += reward;
        } else {
            // The game has ended
            break;
        }
    }

    total_reward
}
