use super::{
    max_depth::MaxDepth,
    searcher::{Evaluation, SearchConstraint},
};
use crate::accumulator::fraction::{Weighted, WeightedAverage};
use crate::accumulator::Accumulator;
use crate::utils;
use std::{
    fmt::Display,
    sync::{Arc, Mutex, MutexGuard},
    time::Instant,
};

pub(super) struct SearchHandle(usize);

pub struct SearchInfo {
    pub constraint: SearchConstraint,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

pub struct Logger {
    pub global_cache_hit_chance_model: Accumulator<MaxDepth, WeightedAverage<f64, f64>>,
    pub cache_hit_depth_model: Accumulator<MaxDepth, WeightedAverage<f64, f64>>,
    pub deadline_miss_model: WeightedAverage<f64, f64>,
    pub search_log: Vec<SearchInfo>,

    pub log_search_results: bool,
    pub print_size_of_critical_structs: bool,
    pub clear_screen: bool,
    pub log_deadline_miss: bool,
    pub print_cache_info: bool,
}

impl Logger {
    pub(super) fn new() -> Self {
        Logger {
            global_cache_hit_chance_model: Accumulator::new(),
            cache_hit_depth_model: Accumulator::new(),
            deadline_miss_model: WeightedAverage::default(),
            search_log: Vec::new(),

            print_size_of_critical_structs: false,
            log_search_results: false,
            clear_screen: false,
            log_deadline_miss: false,
            print_cache_info: false,
        }
    }

    pub(super) fn start_search<S>(&mut self, _s: &S, constraint: SearchConstraint) -> SearchHandle
    where
        S: crate::game::GameState,
    {
        let start_time = Instant::now();

        #[allow(non_snake_case)]
        if self.print_size_of_critical_structs {
            use super::searcher::*;
            use std::mem::size_of;

            let State = size_of_val(_s);
            dbg!(State);
            let Action = size_of::<S::Action>();
            dbg!(Action);

            let Transition = size_of::<super::Transition<S>>();
            dbg!(Transition);

            let Eval = size_of::<Evaluation>();
            dbg!(Eval);
            let EvalAct = size_of::<EvaluatedAction<S::Action>>();
            dbg!(EvalAct);
            let Decision = size_of::<Decision<S>>();
            dbg!(Decision);
            let OptEval = size_of::<OptionEvaluation>();
            dbg!(OptEval);
            let EvalRst = size_of::<EvaluationResult>();
            dbg!(EvalRst);
            let DeciRst = size_of::<DecisionResult<S::Action>>();
            dbg!(DeciRst);

            self.print_size_of_critical_structs = false;
        }

        let search_info = SearchInfo {
            constraint,
            start_time,
            end_time: None,
        };

        if self.log_search_results {
            log::debug!("Searching {constraint}");
        }

        self.search_log.push(search_info);

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

        if self.print_cache_info {
            println!("Hit chance per depth:");
            println!("{:.3}", self.global_cache_hit_chance_model);

            println!("Hit depth per depth:");
            println!("{:.4}", self.cache_hit_depth_model);
        }

        let search_info = match self.search_log.get_mut(search_id) {
            Some(search_info) => search_info,
            None => return,
        };

        search_info.end_time = Some(end_time);

        if !self.log_deadline_miss {
            return;
        }

        let deadline = search_info.constraint.deadline;
        let Some(deadline) = deadline else { return };

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
        self.deadline_miss_model += Weighted::<f64, f64>::new(miss_seconds);
        // } else {
        //     eprintln!(
        //         "Ignoring miss since it has a high error ({miss_duration:.1?}>{outlier_threshold:.1?})",
        //         miss_duration = Duration::from_secs_f64(miss_err),
        //     );
        // }

        // TODO: We should probably be using chrono
        let miss_duration = utils::get_signed_duration(miss_seconds);
        println!("Deadline missed by {miss_duration:?}");

        let avg_miss_seconds = self.deadline_miss_model.clone().evaluate();
        let avg_miss = utils::get_signed_duration(avg_miss_seconds);
        println!("Avg miss: {avg_miss:?}");
    }
}

pub struct LoggerHandle {
    pub logger: Arc<Mutex<Logger>>,
    pub log_cache_info: bool,
}

impl LoggerHandle {
    pub fn new(logger: Arc<Mutex<Logger>>) -> Self {
        Self {
            logger,

            log_cache_info: false,
        }
    }

    fn logger(&mut self) -> MutexGuard<Logger> {
        self.logger.lock().unwrap_or_else(|e| {
            panic!("Failed to acquire lock on Logger: {e}");
        })
    }

    pub(super) fn register_cache_hit(&mut self, depth: MaxDepth, eval: &Evaluation) {
        if !self.log_cache_info {
            return;
        }

        let hit = Weighted::<f64, f64>::new(1.0);
        let mut logger = self.logger();
        logger.global_cache_hit_chance_model.accumulate(depth, hit);

        if let MaxDepth::Bounded(_) = eval.min_depth {
            let hit_depth = Weighted::<f64, f64>::new(eval.min_depth.max_u8().into());
            logger.cache_hit_depth_model.accumulate(depth, hit_depth);
        }
    }

    pub(super) fn register_cache_miss(&mut self, depth: MaxDepth) {
        if !self.log_cache_info {
            return;
        }

        let miss = Weighted::<f64, f64>::new(0.0);
        self.logger()
            .global_cache_hit_chance_model
            .accumulate(depth, miss);
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
}
