use std::{cmp::Ordering, ops};

#[derive(Copy, Clone, Debug, Default)]
pub struct Prioritized<V, P = ()> {
    pub value: V,
    pub priority: P,
}

impl<V: ops::Add<Output = V>, P: Ord> ops::Add for Prioritized<V, P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self.priority.cmp(&rhs.priority) {
            Ordering::Less => rhs,
            Ordering::Equal => Prioritized {
                value: self.value + rhs.value,
                priority: self.priority,
            },
            Ordering::Greater => self,
        }
    }
}

impl<V: ops::AddAssign, P: Ord> ops::AddAssign for Prioritized<V, P> {
    fn add_assign(&mut self, rhs: Self) {
        match self.priority.cmp(&rhs.priority) {
            Ordering::Less => *self = rhs,
            Ordering::Equal => self.value += rhs.value,
            Ordering::Greater => {}
        }
    }
}
