mod orientation;

mod test;

pub use orientation::Orientation;

use crate::generic::{self, moves::AsMove, Movable, Move};

use super::{
    moves::{rotation::Rotatable, ExtendedAxisMove},
    CubeN,
};

/// A cube that can be oriented. This means that you can rotate it, do slice moves and such.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orientable<C: CubeN>
where
    Self: Movable<ExtendedAxisMove>,
{
    /// The original cube
    pub base_cube: C,

    /// The orientation of the cube
    pub orientation: Orientation,
}

impl<C: CubeN + 'static> Orientable<C>
where
    Self: Movable<ExtendedAxisMove>,
{
    /// Creates a new orientable cube with the default orientation.
    pub const fn new(cube: C) -> Self {
        Self {
            base_cube: cube,
            orientation: Orientation::const_default(),
        }
    }
}

// Trait because otherwise we can't implement this method
trait IntoOrientable
where
    Self: CubeN + 'static,
    Orientable<Self>: Movable<ExtendedAxisMove>,
{
    fn orientable(self) -> Orientable<Self> {
        Orientable::new(self)
    }
}

impl<C: CubeN + 'static> IntoOrientable for C where Orientable<C>: Movable<ExtendedAxisMove> {}

impl<M: Move + Rotatable, C: CubeN + Movable<M>> Movable<M> for Orientable<C>
where
    Orientable<C>: Movable<ExtendedAxisMove>,
{
    fn apply(&mut self, m: &M) {
        let mut m = m.clone();

        for rotation in self.orientation.rotations() {
            m.rotate(&rotation);
        }

        self.base_cube.apply(&m);
    }
}

impl<C: CubeN + 'static> generic::Cube for Orientable<C>
where
    Orientable<C>: Movable<ExtendedAxisMove>,
{
    const SOLVED: Self = Orientable::new(C::SOLVED);
}

impl<C: CubeN> AsMove for Orientable<C>
where
    Orientable<C>: Movable<ExtendedAxisMove>,
{
    type Move = ExtendedAxisMove;
}

impl<C: CubeN + 'static> From<C> for Orientable<C>
where
    Orientable<C>: Movable<ExtendedAxisMove>,
{
    fn from(value: C) -> Self {
        Self::new(value)
    }
}
