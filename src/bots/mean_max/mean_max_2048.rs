use super::{EvaluatedAction, Evaluation, MeanMax, SearchConstraint, SearchError};
use crate::{
    board::{Direction, StateOf2048},
    bots::{heuristic, model::weighted::Weighted},
    game::Transition,
};
use std::time::{Duration, Instant};

pub type Action = Direction;
pub type OptionEvaluation = Option<Evaluation>;
pub type EvaluationResult = Result<Evaluation, SearchError>;
pub type Decision = Option<EvaluatedAction<Action>>;
pub type DecisionResult = Result<Decision, SearchError>;

impl<const ROWS: usize, const COLS: usize>
    MeanMax<StateOf2048<ROWS, COLS>, heuristic::PreprocessedBoard>
{
    fn train_model(&mut self, state: &StateOf2048<ROWS, COLS>, eval: Evaluation) {
        let preprocessed_board = heuristic::preprocess_board(state);
        let prev_eval = self.model.entry(preprocessed_board).or_default();

        let decay = 0.995;
        prev_eval.total_value *= decay;
        prev_eval.total_weight *= decay;

        let weight = 2.0_f64.powi(eval.depth.into()) as heuristic::Eval;
        *prev_eval += Weighted::new_weighted(eval.value, weight);
    }

    fn heuristic(&self, state: &StateOf2048<ROWS, COLS>) -> heuristic::Eval {
        // Preprocess the board for the model
        let preprocessed = heuristic::preprocess_board(state);

        self.model.get(&preprocessed).map_or_else(
            || heuristic::heuristic(preprocessed),
            |&weighted| weighted.average_value(),
        )
    }

    pub fn evaluate_state(&mut self, state: &StateOf2048<ROWS, COLS>) -> EvaluationResult {
        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return Err(SearchError::TimeOut);
            }
        }

        let eval = self
            .make_decision(state)?
            .map_or(Evaluation::TERMINAL, |e| e.eval);

        Ok(eval)
    }

    pub fn make_decision(&mut self, state: &StateOf2048<ROWS, COLS>) -> DecisionResult {
        let mut best: Decision = None;

        for transition in state.iter_transitions() {
            let eval = self.evaluate_transition(transition)?;
            let action = transition.action;

            if !best.is_some_and(|best: _| best.eval > eval) {
                best = Some(EvaluatedAction { eval, action });
            }
        }

        Ok(best)
    }

    pub fn decide_until(
        &mut self,
        state: &StateOf2048<ROWS, COLS>,
        constraint: SearchConstraint,
    ) -> Decision {
        let search_id = self.logger.register_search_start(state, constraint);

        // Initial search depth
        self.depth_limit = match constraint.deadline {
            // If there is deadline start at depth 0 and go deeper
            Some(_) => super::Bound::new(0),
            // Else, search with the maximum depth
            None => constraint.max_depth,
        };

        // Remove the previous deadline for the initial search
        self.deadline = None;

        let mut decision: Decision = self
            .make_decision(state)
            .expect("searching with no constraint");

        self.deadline = constraint
            .deadline
            // Bring back the deadline to account for roll-up time
            .map(|deadline| deadline - Duration::from_micros(2));

        // Search deeper loop
        loop {
            self.logger.register_search_result(&search_id, &decision);

            let Some(last_decision) = decision else { break };

            // Reached the max_depth, quit
            if last_decision.eval.fits_depth_bound(constraint.max_depth) {
                break;
            }

            // Move the depth limit higher for a deeper search
            self.depth_limit = last_decision.eval.depth_bound() + 1;

            let best_action: DecisionResult = self.make_decision(state);
            let Ok(new_decision) = best_action else { break };
            decision = new_decision;
        }

        self.logger.register_search_end(search_id);
        decision
    }

    fn cached_evaluation(&mut self, state: &StateOf2048<ROWS, COLS>) -> OptionEvaluation {
        let mut cached_eval = self.evaluation_cache.get(state);

        if cached_eval.is_some_and(|eval| !eval.fits_depth_bound(self.depth_limit)) {
            cached_eval = None;
        }

        self.logger
            .register_lookup_result(cached_eval, self.depth_limit);

        cached_eval.copied()
    }

    fn evaluate_transition(
        &mut self,
        transition: Transition<StateOf2048<ROWS, COLS>, Action>,
    ) -> EvaluationResult {
        let next_state = transition.next_state;

        let Some(eval_depth_limit) = self.depth_limit - 1 else {
            return Ok(Evaluation {
                value: self.heuristic(&next_state) + transition.reward,
                depth: 0,
                is_complete: false,
            });
        };

        if let Some(eval) = self.cached_evaluation(&next_state) {
            return Ok(eval);
        }

        self.depth_limit = eval_depth_limit;

        let mut transition_value = Weighted::<_, super::Value>::default();
        let mut best = Evaluation::TERMINAL;

        for (next_state, weight) in next_state.spawns() {
            let eval = self.evaluate_state(&next_state)?;

            (best.is_complete, best.depth) = std::cmp::min(
                (eval.is_complete, eval.depth),
                (best.is_complete, best.depth),
            );

            transition_value += Weighted::new_weighted(eval.value, weight.into());
        }

        best.value = transition_value.average_value() + transition.reward;
        best.depth = best.depth.saturating_add(1);

        self.evaluation_cache.put(next_state, best);
        if best.depth > 2 {
            self.train_model(&next_state, best);
        }

        self.depth_limit += 1;

        Ok(best)
    }
}
