pub mod board;

use crate::accumulator::fraction::Weighted;
use board::{Cells, Direction};
use rand::distributions::{Distribution as _, WeightedError, WeightedIndex};
use std::fmt::{self, Debug, Display};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct State<const COLS: usize, const ROWS: usize> {
    pub cells: Cells<COLS, ROWS>,
}

impl<const COLS: usize, const ROWS: usize> State<COLS, ROWS> {
    pub fn new() -> Self {
        let cells = Cells::new();
        // PERF: Don't generate all the possible states beforehand
        let options: Vec<_> = board::Spawns::new(cells).collect();
        let weights = options.iter().map(|weighted| weighted.weight.get());
        let dist = WeightedIndex::new(weights).unwrap();
        let mut rng = rand::thread_rng();
        let index = dist.sample(&mut rng);

        Self::from_cells(options[index].value)
    }

    pub fn from_cells<C>(cells: C) -> Self
    where
        Cells<COLS, ROWS>: From<C>,
    {
        Self {
            cells: Cells::from(cells),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> super::GameState for State<COLS, ROWS> {
    type Outcome = Outcome<COLS, ROWS>;
    type Action = Direction;
    type Reward = f32;

    fn outcome(self, action: Self::Action) -> (Self::Reward, Self::Outcome) {
        let mut cells = self.cells;
        let mut reward = 1.0; // TODO: Calculate the actual reward.

        if !cells.swipe(action) {
            // This action didn't change the board, so is not a valid action.
            cells = board::Cells::new();
            reward = 0.0;
        }

        (reward, Outcome { cells })
    }

    fn is_terminal(&self) -> bool {
        self.cells.is_lost()
    }
}

impl<const ROWS: usize, const COLS: usize> Display for State<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.cells, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Outcome<const COLS: usize, const ROWS: usize> {
    pub(crate) cells: Cells<COLS, ROWS>,
}

impl<const COLS: usize, const ROWS: usize> Display for Outcome<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.cells, f)
    }
}

impl<const COLS: usize, const ROWS: usize> IntoIterator for Outcome<COLS, ROWS> {
    type Item = Weighted<State<COLS, ROWS>, board::Weight>;
    type IntoIter = std::iter::Map<
        board::Spawns<COLS, ROWS>,
        fn(<board::Spawns<COLS, ROWS> as Iterator>::Item) -> Self::Item,
    >;

    fn into_iter(self) -> Self::IntoIter {
        let spawns = if self.cells == Cells::new() {
            board::Spawns::empty()
        } else {
            board::Spawns::new(self.cells)
        };

        spawns.map(|weighted| weighted.map(State::from_cells))
    }
}

impl<const COLS: usize, const ROWS: usize> super::DiscreteDistribution for Outcome<COLS, ROWS> {
    type T = State<COLS, ROWS>;
    type Weight = board::Weight;
}

impl<const COLS: usize, const ROWS: usize> super::Outcome<State<COLS, ROWS>>
    for Outcome<COLS, ROWS>
{
    fn collapse(self) -> State<COLS, ROWS> {
        let into_iter = self.clone().into_iter();
        let (min, _max) = into_iter.size_hint();
        let mut weights = Vec::with_capacity(min);
        let mut items = Vec::with_capacity(min);

        for weighted in into_iter {
            weights.push(weighted.weight.get());
            items.push(weighted.value);
        }

        match WeightedIndex::new(weights) {
            Ok(weighted_index) => {
                let idx = weighted_index.sample(&mut rand::thread_rng());
                items.swap_remove(idx)
            }
            Err(WeightedError::NoItem) => State::from_cells(Cells::new()),
            Err(err) => panic!(
                "Failed to collapse outcome: {err}\noutcome:\n{}",
                &self.cells
            ),
        }
    }
}
