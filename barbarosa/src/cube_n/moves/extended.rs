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

/// Fancier moves that only work on [`Orientable`] cubes. 
/// 
/// In regular NxN cubes, you can only do [`AxisMove`]s and [`WideAxisMove`]s. However, with orientable
/// cubes you can use [`ExtendedAxisMove`]s which are basically any move you can imagine on an NxN.
/// 
/// Look at the variants of the enum for specifics. 
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendedAxisMove {
    /// A good ol' regular [`AxisMove`].
    Regular(AxisMove),

    /// A rotation of the cube ("x", "y" or "z").
    Rotation(AxisRotation),

    /// A [`WideAxisMove`]. However, it works for any combination of depth and cubes. 
    /// 
    /// A move with more depth than the side length of the cube is equivalent to a rotation (e.g.: 4Rw in a 3x3 is just X).
    Wide(WideAxisMove<{ u32::MAX }>),

    /// A slice move.
    #[allow(missing_docs)]
    Slice { rot: AxisRotation, wide: bool },
}

impl Move for ExtendedAxisMove {
    fn inverse(&self) -> Self {
        match self {
            Self::Regular(m) => Self::Regular(m.inverse()),
            Self::Rotation(m) => Self::Rotation(m.inverse()),
            Self::Wide(m) => Self::Wide(m.inverse()),
            Self::Slice { rot, wide } => Self::Slice {
                rot: rot.inverse(),
                wide: *wide,
            },
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
                            self.orientation.rotate(&AxisRotation::from(&m.axis_move));
                            opposite(&m, depth_oppossite)
                        };

                        let m = m.set_max_depth::<{ <$cube>::N / 2 - 1 }>().expect("TODO");
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

impl From<AxisMove> for ExtendedAxisMove {
    fn from(m: AxisMove) -> Self {
        Self::Regular(m)
    }
}

impl<const N: u32> From<WideAxisMove<N>> for ExtendedAxisMove {
    fn from(value: WideAxisMove<N>) -> Self {
        let widest = value
            .set_max_depth::<{ u32::MAX }>()
            .expect("`u32::MAX` is bigger or equal to than any N");
        Self::Wide(widest)
    }
}

impl From<AxisRotation> for ExtendedAxisMove {
    fn from(value: AxisRotation) -> Self {
        Self::Rotation(value)
    }
}

impl From<(AxisRotation, bool)> for ExtendedAxisMove {
    fn from((rot, wide): (AxisRotation, bool)) -> Self {
        Self::Slice { rot, wide }
    }
}
