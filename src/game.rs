use std::fmt::{self, Display};

use crate::board::{self, Direction, StateOf2048};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transition<Action, Reward, State> {
    pub action: Action,
    pub reward: Reward,
    pub next: State,
}

type Reward = f32;
type Action = Direction;

// TODO: give this a better name
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Swipe2048<const COLS: usize, const ROWS: usize> {
    pub state: StateOf2048<COLS, ROWS>,
}

impl<const ROWS: usize, const COLS: usize> Swipe2048<ROWS, COLS> {
    const ACTIONS: &'static [Action] = Action::ALL;

    pub fn create() -> Self {
        Swipe2048 {
            state: StateOf2048::new().random_spawn(),
        }
    }

    pub fn full_step(&mut self, action: Action) -> Option<Reward> {
        let Transition { reward, next, .. } = self.half_step(action)?;

        self.state = next.random_spawn();

        (!self.terminal()).then_some(reward)
    }

    pub fn half_step(
        self,
        action: Action,
    ) -> Option<Transition<Action, Reward, Spawn2048<ROWS, COLS>>> {
        let mut state = self.state;
        if !state.swipe(action) {
            return None;
        }

        // TODO: replace with the actual reward
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
    ) -> impl Iterator<Item = Transition<Action, Reward, Spawn2048<ROWS, COLS>>> + '_ {
        let possible_actions = (!self.terminal())
            .then_some(Self::ACTIONS)
            .into_iter()
            .flatten();

        possible_actions.filter_map(|action| self.half_step(*action))
    }
}

impl<const ROWS: usize, const COLS: usize> Display for Swipe2048<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.state.fmt(f)
    }
}

impl<const ROWS: usize, const COLS: usize> From<[[board::Cell; COLS]; ROWS]>
    for Swipe2048<COLS, ROWS>
{
    fn from(cells: [[board::Cell; COLS]; ROWS]) -> Self {
        Swipe2048 {
            state: cells.into(),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> From<StateOf2048<COLS, ROWS>> for Swipe2048<COLS, ROWS> {
    fn from(state: StateOf2048<COLS, ROWS>) -> Self {
        Self { state }
    }
}

pub type Spawn2048<const ROWS: usize, const COLS: usize> = StateOf2048<ROWS, COLS>;
