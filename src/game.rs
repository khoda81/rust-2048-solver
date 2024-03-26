#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transition<Action, Reward, State> {
    pub action: Action,
    pub reward: Reward,
    pub next: State,
}

pub mod twenty_forty_eight;
