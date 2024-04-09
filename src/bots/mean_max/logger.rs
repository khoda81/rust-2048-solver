use super::{max_depth::MaxDepth, Evaluation, SearchConstraint};
use crate::accumulator::{weighted::Weighted, Accumulator};
use crate::utils;
use std::{fmt::Display, time::Instant};

pub(super) struct SearchHandle(usize);

pub struct SearchInfo {
    pub constraint: SearchConstraint,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

pub struct Logger {
    pub cache_hit_chance_model: Accumulator<MaxDepth, Weighted>,
    pub cache_hit_depth_model: Accumulator<MaxDepth, Weighted>,
    pub deadline_miss_model: Weighted,
    pub search_log: Vec<SearchInfo>,

    // Config
    pub log_search_results: bool,
    pub log_cache_info: bool,
    pub log_deadline_miss: bool,
    pub clear_screen: bool,
    pub print_size_of_critical_structs: bool,
}

impl Logger {
    pub(super) fn new() -> Self {
        Logger {
            cache_hit_chance_model: Accumulator::new(),
            cache_hit_depth_model: Accumulator::new(),
            deadline_miss_model: Weighted::default(),
            search_log: Vec::new(),

            log_search_results: false,
            log_cache_info: false,
            log_deadline_miss: false,
            clear_screen: false,
            print_size_of_critical_structs: false,
        }
    }

    pub(super) fn register_cache_hit(&mut self, depth: MaxDepth, eval: &Evaluation) {
        if !self.log_cache_info {
            return;
        }

        let hit = Weighted::new(1.0);
        self.cache_hit_chance_model.accumulate(depth, hit);

        if let MaxDepth::Bounded(_) = eval.min_depth {
            let hit_depth = Weighted::new(eval.min_depth.max_u8().into());
            self.cache_hit_depth_model.accumulate(depth, hit_depth);
        }
    }

    pub(super) fn register_cache_miss(&mut self, depth: MaxDepth) {
        if !self.log_cache_info {
            return;
        }

        let miss = Weighted::new(0.0);
        self.cache_hit_chance_model.accumulate(depth, miss);
    }

    pub(super) fn register_lookup_result(
        &mut self,
        result: Option<&Evaluation>,
        depth_limit: MaxDepth,
    ) {
        match result {
            Some(eval) => self.register_cache_hit(depth_limit, eval),
            None => self.register_cache_miss(depth_limit),
        }
    }

    pub(super) fn start_search<S>(&mut self, _s: &S, constraint: SearchConstraint) -> SearchHandle
    where
        S: crate::game::GameState,
    {
        let start_time = Instant::now();

        #[allow(non_snake_case)]
        if self.print_size_of_critical_structs {
            let State = std::mem::size_of_val(_s);
            dbg!(State);
            let Action = std::mem::size_of::<S::Action>();
            dbg!(Action);

            let Transition = std::mem::size_of::<super::Transition<S>>();
            dbg!(Transition);

            let Eval = std::mem::size_of::<super::Evaluation>();
            dbg!(Eval);
            let EvalAct = std::mem::size_of::<super::EvaluatedAction<S::Action>>();
            dbg!(EvalAct);
            let Decision = std::mem::size_of::<super::Decision<S>>();
            dbg!(Decision);
            let OptEval = std::mem::size_of::<super::OptionEvaluation>();
            dbg!(OptEval);
            let EvalRst = std::mem::size_of::<super::EvaluationResult>();
            dbg!(EvalRst);
            let DeciRst = std::mem::size_of::<super::DecisionResult<S::Action>>();
            dbg!(DeciRst);

            self.print_size_of_critical_structs = false;
        }

        let search_info = SearchInfo {
            constraint,
            start_time,
            end_time: None,
        };

        self.search_log.push(search_info);

        if self.log_search_results {
            log::debug!("Searching {constraint}");
        }

        SearchHandle(self.search_log.len() - 1)
    }

    pub(super) fn register_search_result<D: Display>(
        &mut self,
        &SearchHandle(search_id): &SearchHandle,
        decision: D,
    ) {
        if self.log_search_results {
            if let Some(search_info) = self.search_log.get(search_id) {
                let duration = utils::HumanDuration(search_info.start_time.elapsed());
                // TODO: Print time since previous
                log::debug!("{decision:.2} in {duration:>5}");
            } else {
                log::debug!("{decision:.2}");
            }
        }
    }

    pub(super) fn end_search(&mut self, SearchHandle(search_id): SearchHandle) {
        let end_time = Instant::now();

        if self.log_search_results {
            println!();
        }

        if self.clear_screen {
            print!("\x1b[2J\x1b[H");
        }

        let search_info = match self.search_log.get_mut(search_id) {
            Some(search_info) => search_info,
            None => return,
        };

        search_info.end_time = Some(end_time);

        if self.log_cache_info {
            println!("Hit chance per depth:");
            println!("{:.3}", self.cache_hit_chance_model);

            println!("Hit depth per depth:");
            println!("{:.4}", self.cache_hit_depth_model);
        }

        if !self.log_deadline_miss {
            return;
        }

        let deadline = match search_info.constraint.deadline {
            Some(deadline) => deadline,
            _ => return,
        };

        let miss_seconds = if deadline <= end_time {
            (end_time - deadline).as_secs_f64()
        } else {
            -(deadline - end_time).as_secs_f64()
        };

        // BUG: Disabling outlier detection for now.

        // let avg_miss_seconds = self.deadline_miss_model.weighted_average();
        // let miss_err = (avg_miss_seconds - miss_seconds).abs();
        // let outlier_threshold = Duration::from_micros(5);
        // BUG: This can be thrown off if a high miss happens at the start.

        // if miss_err.is_nan() || Duration::from_secs_f64(miss_err) <= outlier_threshold {
        self.deadline_miss_model += Weighted::new(miss_seconds);
        // } else {
        //     eprintln!(
        //         "Ignoring miss since it has a high error ({miss_duration:.1?}>{outlier_threshold:.1?})",
        //         miss_duration = Duration::from_secs_f64(miss_err),
        //     );
        // }

        let miss_duration = utils::get_signed_duration(miss_seconds);
        println!("Deadline missed by {miss_duration:?}");

        let avg_miss_seconds = self.deadline_miss_model.weighted_average();
        let avg_miss = utils::get_signed_duration(avg_miss_seconds);
        println!("Avg miss: {avg_miss:?}");
    }
}
