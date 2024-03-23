use super::{
    max_depth::{self, MaxDepth},
    mean_max_2048, Evaluation, SearchConstraint,
};
use crate::bots::model::{weighted::Weighted, AccumulationModel};
use crate::utils;
use std::time::{Duration, Instant};

pub(super) struct SearchID(usize);

pub struct SearchInfo {
    pub constraint: SearchConstraint,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

pub struct Logger {
    pub cache_hit_chance_model: AccumulationModel<MaxDepth, Weighted>,
    pub cache_hit_depth_model: AccumulationModel<MaxDepth, Weighted>,
    pub deadline_miss_model: Weighted,
    pub search_log: Vec<SearchInfo>,

    // Config
    pub log_search_results: bool,
    pub log_cache_info: bool,
    pub clear_screen: bool,
    pub show_size_of_critical_structs: bool,
}

impl Logger {
    pub(super) fn new() -> Self {
        Logger {
            cache_hit_chance_model: AccumulationModel::new(),
            cache_hit_depth_model: AccumulationModel::new(),
            deadline_miss_model: Weighted::default(),
            search_log: Vec::new(),

            log_search_results: false,
            log_cache_info: false,
            clear_screen: false,
            show_size_of_critical_structs: false,
        }
    }

    pub(super) fn register_cache_hit(&mut self, depth: MaxDepth, eval: &Evaluation) {
        if !self.log_cache_info {
            return;
        }

        let hit = Weighted::new(1.0);
        self.cache_hit_chance_model.add_to(depth, hit);

        if let MaxDepth::Bounded(_) = eval.min_depth {
            let hit_depth = Weighted::new(eval.min_depth.max_u8().into());
            self.cache_hit_depth_model.add_to(depth, hit_depth);
        }
    }

    pub(super) fn register_cache_miss(&mut self, depth: MaxDepth) {
        if !self.log_cache_info {
            return;
        }

        let miss = Weighted::new(0.0);
        self.cache_hit_chance_model.add_to(depth, miss);
    }

    pub(super) fn register_lookup_result(
        &mut self,
        result: Option<&Evaluation>,
        depth_limit: max_depth::MaxDepth,
    ) {
        match result {
            Some(eval) => self.register_cache_hit(depth_limit, eval),
            None => self.register_cache_miss(depth_limit),
        }
    }

    pub(super) fn register_search_start<T>(
        &mut self,
        _state: &T,
        constraint: SearchConstraint,
    ) -> SearchID {
        let start_time = Instant::now();

        #[allow(non_snake_case)]
        if self.show_size_of_critical_structs {
            let State = std::mem::size_of_val(_state);
            dbg!(State);
            let Action = std::mem::size_of::<mean_max_2048::Action>();
            dbg!(Action);

            let Transition = std::mem::size_of::<
                crate::game::Transition<T, mean_max_2048::Action, super::Value>,
            >();
            dbg!(Transition);

            let Eval = std::mem::size_of::<super::Evaluation>();
            dbg!(Eval);
            let OptEval = std::mem::size_of::<mean_max_2048::OptionEvaluation>();
            dbg!(OptEval);
            let EvalRst = std::mem::size_of::<mean_max_2048::EvaluationResult>();
            dbg!(EvalRst);
            let EvalAct = std::mem::size_of::<super::EvaluatedAction<mean_max_2048::Action>>();
            dbg!(EvalAct);
            let Decision = std::mem::size_of::<mean_max_2048::Decision>();
            dbg!(Decision);
            let DeciRst = std::mem::size_of::<mean_max_2048::DecisionResult>();
            dbg!(DeciRst);

            self.show_size_of_critical_structs = false;
        }

        let search_info = SearchInfo {
            constraint,
            start_time,
            end_time: None,
        };

        self.search_log.push(search_info);

        if self.log_search_results {
            // TODO: Use a logging library.

            println!("Searching {constraint}");
        }

        SearchID(self.search_log.len() - 1)
    }

    pub(super) fn register_search_result(
        &mut self,
        &SearchID(search_index): &SearchID,
        decision: &mean_max_2048::Decision,
    ) {
        if self.log_search_results {
            if let Some(result) = decision {
                print!("{result:.2}");
            } else {
                print!("terminate");
            }

            if let Some(search_info) = self.search_log.get(search_index) {
                let duration = utils::HumanDuration(search_info.start_time.elapsed());
                print!(" in {duration:>5}");
            }

            println!()
        }
    }

    pub(super) fn register_search_end(&mut self, SearchID(search_index): SearchID) {
        let end_time = Instant::now();

        if self.log_search_results {
            println!();
        }

        if self.clear_screen {
            print!("\x1b[2J\x1b[H");
        }

        let search_info = match self.search_log.get_mut(search_index) {
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

        let deadline = match search_info.constraint.deadline {
            Some(deadline) => deadline,
            _ => return,
        };

        let miss_seconds = if deadline <= end_time {
            (end_time - deadline).as_secs_f64()
        } else {
            -(deadline - end_time).as_secs_f64()
        };

        let avg_miss_seconds = self.deadline_miss_model.weighted_average();
        let miss_err = (avg_miss_seconds - miss_seconds).abs();
        let outlier_threshold = Duration::from_micros(5);
        if miss_err.is_nan() || Duration::from_secs_f64(miss_err) <= outlier_threshold {
            self.deadline_miss_model += Weighted::new(miss_seconds);
        } else {
            // FIX: This can be thrown off if a high miss happens at the start.
            // TODO: Loop over all the searches every time.
            eprintln!(
                "Ignoring miss since it has a high error ({miss_duration:.1?}>{outlier_threshold:.1?})",
                miss_duration = Duration::from_secs_f64(miss_err),
            );
        }

        let miss_duration = utils::get_signed_duration(miss_seconds);
        println!("Deadline missed by {miss_duration:?}");

        let avg_miss_seconds = self.deadline_miss_model.weighted_average();
        let avg_miss = utils::get_signed_duration(avg_miss_seconds);
        println!("Avg miss: {avg_miss:?}");
    }
}
