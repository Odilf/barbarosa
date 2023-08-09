use std::mem;

use crate::{
    cube3::Cube3,
    cube_n::{
        orientable::Orientable,
        space::{Direction, Face},
        Cube2, Cube4, Cube5, Cube6, Cube7, CubeN,
    },
    generic::{Movable, Move},
};

use super::{
    rotation::{AxisRotation, Rotatable},
    AxisMove, WideAxisMove,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendedAxisMove {
    Regular(AxisMove),
    Rotation(AxisRotation),
    Wide(WideAxisMove<{ u32::MAX }>),
    Slice {
        rot: AxisRotation,
        wide: bool,
    },
}

impl Move for ExtendedAxisMove {
    fn inverse(&self) -> Self {
        match self {
            Self::Regular(m) => Self::Regular(m.inverse()),
            Self::Rotation(m) => Self::Rotation(m.inverse()),
            Self::Wide(m) => Self::Wide(m.inverse()),
            Self::Slice { rot, wide } => Self::Slice { rot: rot.inverse(), wide: *wide },
        }
    }
}

fn opposite<const N: u32>(m: &WideAxisMove<N>, depth: u32) -> WideAxisMove<N> {
    AxisMove::new(m.face().opposite(), m.amount())
        .widen(depth)
        .unwrap()
}

// This has to be a macro because the type system can't enforce that a `CubeN::N` is movable by `WideAxisMove<{N / 2 - 1}>`
macro_rules! impl_movable_extended {
    ([$($cube:ty),*]) => {
        $(
            impl_movable_extended!($cube);
        )*
    };

    ($cube:ty) => {
        impl Movable<ExtendedAxisMove> for Orientable<$cube> {
            fn apply(&mut self, m: &ExtendedAxisMove) {
                match m {
                    ExtendedAxisMove::Regular(m) => self.apply(m),
                    ExtendedAxisMove::Rotation(rot) => self.orientation.rotate(rot),
                    ExtendedAxisMove::Wide(m) => {
                        let Some(depth_oppossite) = <$cube>::N.checked_sub(m.depth()) else {
                            // If depth is more than N, then it is equivalent to a rotation
                            self.orientation.rotate(&AxisRotation::from(&m.axis_move));
                            return;
                        };

                        let m = if m.depth() < depth_oppossite {
                            m.clone()
                        } else {
                            opposite(&m, depth_oppossite)
                        };

                        // Safe because we're changing between wide axis moves which have the same exact structure
                        let m = unsafe { mem::transmute::<_, WideAxisMove<{ <$cube>::N / 2 - 1 }>>(m) };
                        self.base_cube.apply(&m);
                    },
                    ExtendedAxisMove::Slice { rot, wide: _wide } => {
                        for dir in [Direction::Positive, Direction::Negative] {
                            let m = AxisMove::new(Face::new(rot.axis, dir), rot.amount * Direction::Negative);
                            self.apply(&m);
                        }

                        self.orientation.rotate(rot);

                        todo!("Implement wideness of slice moves");
                    },
                }
            }
        }
    };
}

impl_movable_extended!([Cube2, Cube3, Cube4, Cube5, Cube6, Cube7]);
