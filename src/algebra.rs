use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Dihedral<const N: u8> {
    pub flip: bool,
    pub rot: u8,
}

impl<const N: u8> Default for Dihedral<N> {
    fn default() -> Self {
        Self::id()
    }
}

impl<const N: u8> Dihedral<N> {
    pub fn id() -> Self {
        Self {
            flip: false,
            rot: 0,
        }
    }

    pub fn op(&self, other: &Self) -> Self {
        Self {
            flip: self.flip != other.flip,
            rot: if other.flip { N - self.rot } else { self.rot } + other.rot % N,
        }
    }

    pub fn inverse(&self) -> Self {
        Self {
            rot: if self.flip { N - self.rot } else { self.rot },
            flip: self.flip,
        }
    }

    pub fn all() -> DihedralGenerator<N> {
        DihedralGenerator::default()
    }
}

impl Dihedral<4> {
    pub fn is_horizontal(&self) -> bool {
        (self.rot % 2 == 0) ^ self.flip
    }
}

impl<const N: u8> fmt::Display for Dihedral<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = format!("r{}", self.rot);
        if self.flip {
            res = "-".to_string() + &res;
        }

        f.pad(&res)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DihedralGenerator<const N: u8> {
    pub next: Option<Dihedral<N>>,
}

impl<const N: u8> Iterator for DihedralGenerator<N> {
    type Item = Dihedral<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;

        self.next = match current {
            Dihedral { flip: true, rot } if rot + 1 >= N => None,

            Dihedral { flip: true, rot } => Some(Dihedral {
                flip: true,
                rot: rot + 1,
            }),

            Dihedral { flip: false, rot } if rot + 1 >= N => Some(Dihedral { flip: true, rot: 0 }),

            Dihedral { flip: false, rot } => Some(Dihedral {
                flip: false,
                rot: rot + 1,
            }),
        };

        Some(current)
    }
}

impl<const N: u8> Default for DihedralGenerator<N> {
    fn default() -> Self {
        Self {
            next: Some(Default::default()),
        }
    }
}

#[cfg(test)]
mod test_algebra {
    use super::*;

    #[test]
    fn test_dihedral_gen() {
        assert!(Dihedral::<4>::all().count() == 8);
        assert!(Dihedral::<3>::all().count() == 6);
        assert!(Dihedral::<2>::all().count() == 4);
        assert!(Dihedral::<1>::all().count() == 2);
    }
}
