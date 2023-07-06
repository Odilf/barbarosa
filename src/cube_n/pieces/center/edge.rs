use crate::{
    cube_n::{
        moves::rotation::Rotatable,
        pieces::edge::ParallelAxesError,
        space::{faces, Direction, Face},
        WideAxisMove,
    },
    generic,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeCenter {
    pub main_face: Face,
    pub handedness: Direction,
    pub side_direction: Direction,
}

impl generic::Piece for EdgeCenter {}

impl Rotatable for EdgeCenter {
    fn rotate(&mut self, rotation: &crate::cube_n::moves::rotation::AxisRotation) {
        let side_face = self.side_face().rotated(rotation);

        // Very important to rotate face only *after* getting the side face.
        self.main_face.rotate(rotation);

        self.handedness = self
            .main_face
            .axis
            .get_handedness(&side_face.axis)
            .expect("Side face should be perpendicular to main face");

        self.side_direction = side_face.direction;
    }
}

impl EdgeCenter {
    pub const fn new(main_face: Face, handedness: Direction, side_direction: Direction) -> Self {
        Self {
            main_face,
            handedness,
            side_direction,
        }
    }

    pub fn try_from_faces(main_face: Face, side_face: Face) -> Result<Self, ParallelAxesError> {
        let handedness = main_face.axis.get_handedness(&side_face.axis)?;

        Ok(EdgeCenter {
            main_face,
            handedness,
            side_direction: side_face.direction,
        })
    }

    pub fn side_face(&self) -> Face {
        let side_axis = self.main_face.axis.next_with_handedness(self.handedness);

        debug_assert_ne!(side_axis, self.main_face.axis);

        Face::new(side_axis, self.side_direction)
    }

    pub fn is_solved(&self, original: &Self) -> bool {
        self.main_face == original.main_face
    }

    pub fn in_wide_move<const N: u32>(&self, piece_depth: u32, m: &WideAxisMove<N>) -> bool {
        if m.face() == &self.main_face {
            return true;
        }

        if m.face() == &self.side_face() && piece_depth <= N {
            return true;
        }

        false
    }
}

pub const SOLVED: [EdgeCenter; 24] = {
    use faces::*;
    use Direction::*;

    [
        EdgeCenter::new(R, Positive, Positive),
        EdgeCenter::new(R, Positive, Negative),
        EdgeCenter::new(R, Negative, Negative),
        EdgeCenter::new(R, Negative, Positive),
        EdgeCenter::new(U, Positive, Positive),
        EdgeCenter::new(U, Positive, Positive),
        EdgeCenter::new(U, Positive, Negative),
        EdgeCenter::new(U, Positive, Negative),
        EdgeCenter::new(F, Negative, Negative),
        EdgeCenter::new(F, Negative, Negative),
        EdgeCenter::new(F, Negative, Positive),
        EdgeCenter::new(F, Negative, Positive),
        EdgeCenter::new(L, Positive, Positive),
        EdgeCenter::new(L, Positive, Negative),
        EdgeCenter::new(L, Negative, Negative),
        EdgeCenter::new(L, Negative, Positive),
        EdgeCenter::new(D, Positive, Positive),
        EdgeCenter::new(D, Positive, Positive),
        EdgeCenter::new(D, Positive, Negative),
        EdgeCenter::new(D, Positive, Negative),
        EdgeCenter::new(B, Negative, Negative),
        EdgeCenter::new(B, Negative, Negative),
        EdgeCenter::new(B, Negative, Positive),
        EdgeCenter::new(B, Negative, Positive),
    ]
};
