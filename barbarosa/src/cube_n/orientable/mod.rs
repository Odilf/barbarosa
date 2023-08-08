mod orientation;

mod test;

pub use orientation::Orientation;

use crate::generic::{Movable, Move};

use super::{moves::rotation::Rotatable, CubeN};

/// A cube that can be oriented. This means that you can rotate it, do slice moves and such.
pub struct Orientable<C: CubeN> {
    /// The original cube
    pub cube: C,

    /// The orientation of the cube
    pub orientation: Orientation,
}

impl<C: CubeN> Orientable<C> {
    /// Creates a new orientable cube with the default orientation.
    pub fn new(cube: C) -> Self {
        Self {
            cube,
            orientation: Orientation::default(),
        }
    }
}

// Trait because otherwise we can't implement this method
trait IntoOrientable
where
    Self: CubeN,
{
    fn orientable(self) -> Orientable<Self> {
        Orientable::new(self)
    }
}

impl<C: CubeN> IntoOrientable for C {}

impl<M: Move + Rotatable, C: CubeN + Movable<M>> Movable<M> for Orientable<C> {
    fn apply(&mut self, m: &M) {
        let mut m = m.clone();

        for rotation in self.orientation.rotations() {
            m.rotate(&rotation);
        }

        self.cube.apply(&m);
    }
}
