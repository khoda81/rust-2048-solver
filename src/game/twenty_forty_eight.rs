type Reward = f32;
type Action = Direction;

use super::Transition;
use crate::board::{Cells, Direction};
use std::fmt::{self, Display};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TwentyFortyEight<const COLS: usize, const ROWS: usize> {
    pub state: Cells<COLS, ROWS>,
}

impl<const ROWS: usize, const COLS: usize> TwentyFortyEight<ROWS, COLS> {
    pub(crate) const ACTIONS: &'static [Action] = Action::ALL;

    pub fn new() -> Self {
        Self::from_state(Cells::new().random_spawn())
    }

    pub fn from_state(state: Cells<ROWS, COLS>) -> Self {
        TwentyFortyEight { state }
    }

    pub fn step(&mut self, action: Action) -> Option<Reward> {
        let Transition { reward, next, .. } = self.half_step(action)?;

        self.state = next.random_spawn();

        (!self.terminal()).then_some(reward)
    }

    // TODO: Add a method that return an iterator of potential future states (with respective weights), then deprecate this.
    pub fn half_step(
        self,
        action: Action,
    ) -> Option<Transition<Action, Reward, Cells<ROWS, COLS>>> {
        let mut state = self.state;
        if !state.swipe(action) {
            return None;
        }

        // TODO: Replace with the actual reward.
        let reward = 1.0;

        Some(Transition {
            action,
            reward,
            next: state,
        })
    }

    pub fn terminal(&self) -> bool {
        self.state.is_lost()
    }

    pub fn transitions(
        &self,
    ) -> impl Iterator<Item = Transition<Action, Reward, Cells<ROWS, COLS>>> + '_ {
        let possible_actions = (!self.terminal())
            .then_some(Self::ACTIONS)
            .into_iter()
            .flatten();

        possible_actions.filter_map(|action| self.half_step(*action))
    }
}

impl<const ROWS: usize, const COLS: usize> Display for TwentyFortyEight<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.state.fmt(f)
    }
}
