use crate::bots::heuristic::TwentyFortyEightHeuristic;
use crate::game::twenty_forty_eight::State;
use std::sync::{Arc, Mutex};

impl<const ROWS: usize, const COLS: usize>
    super::MeanMax<State<COLS, ROWS>, TwentyFortyEightHeuristic<COLS, ROWS>>
{
    pub fn new() -> Self {
        let (result_sender, result_receiver) = std::sync::mpsc::channel();

        let mut this = Self {
            logger: Arc::new(Mutex::new(super::logger::Logger::new())),
            heuristic: std::marker::PhantomData,

            searcher_threads: Vec::new(),
            result_receiver,
            result_sender,
        };

        let num_threads = std::thread::available_parallelism()
            .ok()
            //.and_then(|threads| std::num::NonZeroUsize::new(threads.get() - 1))
            .unwrap_or(std::num::NonZeroUsize::MIN);

        (0..num_threads.get()).for_each(|_| this.add_searcher());
        this
    }
}

impl<const ROWS: usize, const COLS: usize> Default
    for super::MeanMax<State<COLS, ROWS>, TwentyFortyEightHeuristic<COLS, ROWS>>
{
    fn default() -> Self {
        Self::new()
    }
}
