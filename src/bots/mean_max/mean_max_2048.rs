use super::{EvaluatedAction, Evaluation, MeanMax, SearchConstraint, SearchError, Value};
use crate::{
    board::{Direction, StateOf2048},
    bots::{heuristic, model::weighted::Weighted},
    game,
};
use std::time::{Duration, Instant};

pub type Action = Direction;
pub type State<const ROWS: usize, const COLS: usize> = StateOf2048<ROWS, COLS>;

pub type Transition<const ROWS: usize, const COLS: usize> =
    game::Transition<State<ROWS, COLS>, Action, Value>;

pub type OptionEvaluation = Option<Evaluation>;
pub type EvaluationResult = Result<Evaluation, SearchError>;
pub type Decision = Option<EvaluatedAction<Action>>;
pub type DecisionResult = Result<Decision, SearchError>;

impl<const ROWS: usize, const COLS: usize>
    MeanMax<State<ROWS, COLS>, heuristic::PreprocessedBoard>
{
    fn train_model(&mut self, state: &State<ROWS, COLS>, eval: Evaluation) {
        let preprocessed_board = heuristic::preprocess_board(state);
        let prev_eval = self.model.entry(preprocessed_board).or_default();

        let decay = 0.995;
        prev_eval.total_value *= decay;
        prev_eval.total_weight *= decay;

        // TODO find a better solution for this
        // let weight = 2.0_f64.powi(eval.depth.into()) as heuristic::Eval;
        let weight = 1.0;
        *prev_eval += Weighted::new_weighted(eval.value as f64, weight);
    }

    fn evaluate_with_model(&self, transition: &Transition<ROWS, COLS>) -> Value {
        // Preprocess the board for the model
        let preprocessed = heuristic::preprocess_board(&transition.next_state);

        let next_state_eval = self.model.get(&preprocessed).map_or_else(
            || heuristic::heuristic(preprocessed),
            |&weighted| weighted.average_value(),
        ) as Value;

        next_state_eval + transition.reward
    }

    pub fn evaluate_state(&mut self, state: &State<ROWS, COLS>) -> EvaluationResult {
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

    pub fn make_decision(&mut self, state: &State<ROWS, COLS>) -> DecisionResult {
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
        state: &State<ROWS, COLS>,
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
            if last_decision.eval.depth >= constraint.max_depth {
                break;
            }

            // Move the depth limit higher for a deeper search
            self.depth_limit = last_decision.eval.depth + 1;

            let best_action: DecisionResult = self.make_decision(state);
            let Ok(new_decision) = best_action else { break };
            decision = new_decision;
        }

        self.logger.register_search_end(search_id);
        decision
    }

    fn cached_evaluation(&mut self, transition: &Transition<ROWS, COLS>) -> OptionEvaluation {
        let mut cached_eval = self.evaluation_cache.get(&transition.next_state).copied();

        if let Some(eval) = cached_eval.as_mut() {
            if eval.depth < self.depth_limit {
                cached_eval = None;
            } else {
                eval.value += transition.reward;
            }
        }

        self.logger
            .register_lookup_result(cached_eval.as_ref(), self.depth_limit);

        cached_eval
    }

    fn evaluate_transition(&mut self, transition: Transition<ROWS, COLS>) -> EvaluationResult {
        let Some(eval_depth_limit) = self.depth_limit - 1 else {
            return Ok(Evaluation {
                value: self.evaluate_with_model(&transition),
                depth: super::Bound::new(0),
            });
        };

        if let Some(eval) = self.cached_evaluation(&transition) {
            return Ok(eval);
        }

        self.depth_limit = eval_depth_limit;

        let mut transition_value = Weighted::<_, Value>::default();
        let mut best = Evaluation {
            value: 0.0,
            depth: super::Bound::Unlimited,
        };

        for (next_state, weight) in transition.next_state.spawns() {
            let eval = self.evaluate_state(&next_state)?;

            best.depth = std::cmp::min(eval.depth, best.depth);
            transition_value += Weighted::new_weighted(eval.value, weight.into());
        }

        best.value = transition_value.average_value() + transition.reward;
        best.depth += 1;

        self.evaluation_cache.put(transition.next_state, best);
        if best.depth.max_u8() > 2 {
            self.train_model(&transition.next_state, best);
        }

        self.depth_limit += 1;
        Ok(best)
    }
}
