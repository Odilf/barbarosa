use crate::cube_n::{
    moves::rotation::{AxisRotation, Rotatable},
    pieces::edge::ParallelAxesError,
    space::{faces, Face},
    WideAxisMove,
};

// Invariant to be upheld: `main_face` and `side_face` must be perpendicular
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeCenter {
    main_face: Face,
    side_face: Face,
}

impl Rotatable for EdgeCenter {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.main_face.rotate(rotation);
        self.side_face.rotate(rotation);
    }
}

pub fn in_wide_move<const N: u32>(
    center: &EdgeCenter,
    piece_depth: u32,
    m: &WideAxisMove<N>,
) -> bool {
    if m.face() == &center.main_face {
        return true;
    }

    if m.face() == &center.side_face && piece_depth <= N {
        return true;
    }

    false
}

impl EdgeCenter {
    pub fn new(main_face: Face, side_face: Face) -> Result<Self, ParallelAxesError> {
        if main_face.axis == side_face.axis {
            return Err(ParallelAxesError::SameAxes([
                main_face.axis,
                side_face.axis,
            ]));
        }

        Ok(Self {
            main_face,
            side_face,
        })
    }

    pub const fn new_unchecked(main_face: Face, side_face: Face) -> Self {
        Self {
            main_face,
            side_face,
        }
    }

    pub fn is_solved(&self, original: &EdgeCenter) -> bool {
        self.main_face == original.main_face
    }
}

pub const SOLVED: [EdgeCenter; 24] = {
    use faces::*;

    [
        EdgeCenter::new_unchecked(R, U),
        EdgeCenter::new_unchecked(R, F),
        EdgeCenter::new_unchecked(R, D),
        EdgeCenter::new_unchecked(R, B),
        EdgeCenter::new_unchecked(U, R),
        EdgeCenter::new_unchecked(U, F),
        EdgeCenter::new_unchecked(U, L),
        EdgeCenter::new_unchecked(U, B),
        EdgeCenter::new_unchecked(F, U),
        EdgeCenter::new_unchecked(F, R),
        EdgeCenter::new_unchecked(F, D),
        EdgeCenter::new_unchecked(F, L),
        EdgeCenter::new_unchecked(L, U),
        EdgeCenter::new_unchecked(L, F),
        EdgeCenter::new_unchecked(L, D),
        EdgeCenter::new_unchecked(L, B),
        EdgeCenter::new_unchecked(D, R),
        EdgeCenter::new_unchecked(D, F),
        EdgeCenter::new_unchecked(D, L),
        EdgeCenter::new_unchecked(D, B),
        EdgeCenter::new_unchecked(B, U),
        EdgeCenter::new_unchecked(B, R),
        EdgeCenter::new_unchecked(B, D),
        EdgeCenter::new_unchecked(B, L),
    ]
};
