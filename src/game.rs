use std::fmt::{self, Display};

use crate::board::{self, Cells, Direction};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transition<Action, Reward, State> {
    pub action: Action,
    pub reward: Reward,
    pub next: State,
}

type Reward = f32;
type Action = Direction;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GameState<const COLS: usize, const ROWS: usize> {
    pub state: Cells<COLS, ROWS>,
}

impl<const ROWS: usize, const COLS: usize> GameState<ROWS, COLS> {
    const ACTIONS: &'static [Action] = Action::ALL;

    pub fn create() -> Self {
        GameState {
            state: Cells::new().random_spawn(),
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

impl<const ROWS: usize, const COLS: usize> Display for GameState<ROWS, COLS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.state.fmt(f)
    }
}

impl<const ROWS: usize, const COLS: usize> From<[[board::Cell; COLS]; ROWS]>
    for GameState<COLS, ROWS>
{
    fn from(cells: [[board::Cell; COLS]; ROWS]) -> Self {
        GameState {
            state: cells.into(),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> From<Cells<COLS, ROWS>> for GameState<COLS, ROWS> {
    fn from(state: Cells<COLS, ROWS>) -> Self {
        Self { state }
    }
}
