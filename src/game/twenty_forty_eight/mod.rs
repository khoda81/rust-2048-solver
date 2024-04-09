pub mod board;

use super::{DiscreteDistribution, GameState, Outcome};
use crate::accumulator::fraction::Weighted;
use board::{Cells, Direction};
use rand::distributions::{Distribution as _, WeightedError, WeightedIndex};
use std::fmt::{self, Debug, Display};

// TODO: Rename this to State?
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TwentyFortyEight<const COLS: usize, const ROWS: usize> {
    pub cells: Cells<COLS, ROWS>,
}

impl<const COLS: usize, const ROWS: usize> TwentyFortyEight<COLS, ROWS> {
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
        TwentyFortyEight {
            cells: Cells::from(cells),
        }
    }
}

impl<const ROWS: usize, const COLS: usize> GameState for TwentyFortyEight<COLS, ROWS> {
    type Outcome = TwentyFortyEightOutcome<COLS, ROWS>;
    type Action = Direction;
    type Reward = f32;

    fn outcome(self, action: Self::Action) -> (Self::Reward, Self::Outcome) {
        let mut cells = self.cells;
        if !cells.swipe(action) {
            // This action didn't change the board, so is not a valid action.
            return (
                0.0,
                TwentyFortyEightOutcome {
                    cells: board::Cells::new(),
                },
            );
        }

        // TODO: Calculate the actual reward.
        let reward = 1.0;

        (reward, TwentyFortyEightOutcome { cells })
    }

    fn is_terminal(&self) -> bool {
        self.cells.is_lost()
    }
}

impl<const ROWS: usize, const COLS: usize> Display for TwentyFortyEight<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.cells, f)
    }
}

// TODO: Rename this to Outcome?
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TwentyFortyEightOutcome<const COLS: usize, const ROWS: usize> {
    pub(crate) cells: Cells<COLS, ROWS>,
}

impl<const COLS: usize, const ROWS: usize> Display for TwentyFortyEightOutcome<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.cells, f)
    }
}

impl<const COLS: usize, const ROWS: usize> IntoIterator for TwentyFortyEightOutcome<COLS, ROWS> {
    type Item = Weighted<TwentyFortyEight<COLS, ROWS>, board::Weight>;
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

        spawns.map(|weighted| weighted.map(TwentyFortyEight::from_cells))
    }
}

impl<const COLS: usize, const ROWS: usize> DiscreteDistribution
    for TwentyFortyEightOutcome<COLS, ROWS>
{
    type T = TwentyFortyEight<COLS, ROWS>;
    type Weight = board::Weight;
}

impl<const COLS: usize, const ROWS: usize> Outcome<TwentyFortyEight<COLS, ROWS>>
    for TwentyFortyEightOutcome<COLS, ROWS>
{
    fn collapse(self) -> TwentyFortyEight<COLS, ROWS> {
        let into_iter = self.clone().into_iter();
        let (min, _max) = into_iter.size_hint();
        let mut weights = Vec::with_capacity(min);
        let mut items = Vec::with_capacity(min);

        for weighted in into_iter {
            weights.push(weighted.weight.get());
            items.push(weighted.value);
        }

        let weighted_index = match WeightedIndex::new(weights) {
            Ok(index) => index,
            Err(WeightedError::NoItem) => return TwentyFortyEight::from_cells(Cells::new()),
            Err(err) => panic!(
                "Failed to collapse outcome: {err}\noutcome:\n{}",
                &self.cells
            ),
        };

        let idx = weighted_index.sample(&mut rand::thread_rng());

        // TODO: why do we need to move out of this?
        items[idx].clone()
    }
}
