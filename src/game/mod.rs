pub mod twenty_forty_eight;

use crate::accumulator::weighted::Weighted;

pub trait Outcome<G: GameState> {
    fn collapse(self) -> G;
}

pub trait GameState: Sized
where
    Self::Outcome: Outcome<Self>,
{
    type Outcome;
    type Action;
    type Reward;

    fn is_terminal(&self) -> bool;
    fn outcome(self, action: Self::Action) -> (Self::Reward, Self::Outcome);
}

pub trait Discrete: Sized {
    fn iter() -> impl Iterator<Item = Self>;
}

pub trait DiscreteDistribution: IntoIterator<Item = Weighted<Self::T, Self::Weight>> {
    type T;
    type Weight;
}
