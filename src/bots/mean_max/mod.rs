pub mod logger;
pub mod max_depth;
pub mod mean_max_2048;
pub mod searcher;

use crate::game;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;

struct Task<Game> {
    task_id: usize,
    state: Game,
    search_constraint: searcher::SearchConstraint,
}

struct SearchResult<Game: game::GameState> {
    task_id: usize,
    state: Game,
    result: searcher::DecisionResult<Game::Action>,
}

pub struct SearcherThread<Game: game::GameState> {
    thread: JoinHandle<()>,
    task_sender: mpsc::Sender<Task<Game>>,
}

// TODO: Add concurrency to cache and search
pub struct MeanMax<Game: game::GameState, Heuristic> {
    pub logger: Arc<Mutex<logger::Logger>>,
    heuristic: PhantomData<Heuristic>,

    //evaluation_cache: lru::LruCache<Game::Outcome, Evaluation>,
    pub searcher_threads: Vec<SearcherThread<Game>>,
    result_receiver: mpsc::Receiver<SearchResult<Game>>,
    result_sender: mpsc::Sender<SearchResult<Game>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Transition<G: game::GameState> {
    pub action: G::Action,
    pub reward: G::Reward,
    pub next: G,
}

impl<G, H> MeanMax<G, H>
where
    H: Default,
    G: game::GameState + Send + Clone + Display + 'static,
    G::Outcome: game::DiscreteDistribution<T = G> + Hash + Eq + Clone + Display,
    G::Action: game::Discrete + Send + Clone + Display,
    searcher::Value: From<G::Reward> + From<<G::Outcome as game::DiscreteDistribution>::Weight>,
    H: super::heuristic::Heuristic<G::Outcome, searcher::Value>,
    <G::Outcome as game::DiscreteDistribution>::Weight: Debug,
{
    const DEFAULT_CACHE_SIZE: usize = 0xF0000;

    pub fn decide_until(
        &mut self,
        state: &G,
        constraint: searcher::SearchConstraint,
    ) -> searcher::Decision<G::Action> {
        let search_handle = self.logger.lock().unwrap().start_search(state, constraint);

        let mut search_constraint = searcher::SearchConstraint {
            // No deadline for the initial search
            deadline: None,
            // Initial search depth
            max_depth: match constraint.deadline {
                // If there is a deadline, start at depth 0 and go deeper
                Some(_) => max_depth::MaxDepth::new(0),
                // Otherwise, search with the maximum depth
                None => constraint.max_depth,
            },
        };

        let mut busy_tasks = HashSet::new();

        for (task_id, searcher) in self.searcher_threads.iter().enumerate() {
            let task = Task {
                task_id,
                search_constraint,
                state: state.clone(),
            };

            search_constraint.deadline = constraint.deadline;

            searcher
                .task_sender
                .send(task)
                .expect("searcher thread should be alive as long as the sender is alive");

            busy_tasks.insert(task_id);

            search_constraint.max_depth += 1;
            search_constraint.max_depth = search_constraint.max_depth.min(constraint.max_depth);
        }

        let mut decision: Option<searcher::Decision<G::Action>> = None;
        let mut search_done = false;

        // Search deeper loop
        while !busy_tasks.is_empty() {
            let SearchResult {
                task_id,
                state: _,
                result,
            } = self
                .result_receiver
                .recv()
                .expect("there should be at least one result sender alive");
            log::trace!("Result from searcher #{task_id}");

            busy_tasks.remove(&task_id);

            let Ok(new_decision) = result else {
                search_done = true;
                continue;
            };

            let mut logger = self.logger.lock().unwrap();
            logger.register_search_result(&search_handle, &new_decision);

            search_constraint.max_depth = search_constraint
                .max_depth
                .max(new_decision.eval().min_depth);

            match decision {
                None => decision = Some(new_decision),
                Some(ref best_decision) => {
                    if new_decision.eval().min_depth > best_decision.eval().min_depth {
                        decision = Some(new_decision);
                    }
                }
            }

            // If last decision was Resign break
            let last_decision = match decision.as_ref().unwrap() {
                searcher::Decision::Act(last_decision) => last_decision,
                searcher::Decision::Resign => {
                    search_done = true;
                    continue;
                }
            };

            // Reached the max_depth, abort
            if last_decision.eval.min_depth >= constraint.max_depth {
                search_done = true;
                continue;
            }

            // Move the depth limit higher for a deeper search
            search_constraint.max_depth += 1;

            if search_done {
                continue;
            }

            let task = Task {
                task_id,
                search_constraint,
                state: state.clone(),
            };

            log::trace!("Scheduling #{task_id} for {search_constraint}");

            self.searcher_threads[task_id]
                .task_sender
                .send(task)
                .expect("searcher thread should be alive as long as the sender is alive");

            busy_tasks.insert(task_id);
        }

        self.logger.lock().unwrap().end_search(search_handle);
        decision.unwrap()
    }

    pub fn add_searcher(&mut self) {
        let (task_sender, task_reciever) = mpsc::channel::<Task<G>>();
        let result_sender = self.result_sender.clone();

        let logger = logger::LoggerHandle::new(self.logger.clone());
        let thread = std::thread::spawn(move || {
            let capacity = Self::DEFAULT_CACHE_SIZE.try_into().unwrap();
            let heuristic = H::default();
            let mut searcher = searcher::Searcher::new(heuristic, capacity, logger);
            while let Ok(task) = task_reciever.recv() {
                let result = searcher.search(task);
                if result_sender.send(result).is_err() {
                    break;
                }
            }
        });

        let searcher = SearcherThread {
            thread,
            task_sender,
        };

        self.searcher_threads.push(searcher);
    }
}
