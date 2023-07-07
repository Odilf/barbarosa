use crate::{
    cube_n::{
        moves::rotation::{AxisRotation, Rotatable},
        pieces::edge::ParallelAxesError,
        space::{faces, Axis, Direction, Face},
        WideAxisMove,
    },
    generic,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CenterEdge {
    pub main_face: Face,
    pub handedness: Direction,
    pub side_direction: Direction,
}

impl generic::Piece for CenterEdge {}

impl Rotatable for CenterEdge {
    fn rotate(&mut self, rotation: &AxisRotation) {
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

impl CenterEdge {
    pub const fn new(main_face: Face, handedness: Direction, side_direction: Direction) -> Self {
        Self {
            main_face,
            handedness,
            side_direction,
        }
    }

    pub fn try_from_faces(main_face: Face, side_face: Face) -> Result<Self, ParallelAxesError> {
        let handedness = main_face.axis.get_handedness(&side_face.axis)?;

        Ok(CenterEdge {
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

    pub fn normal_axis(&self) -> Axis {
        let output = self.main_face.axis.next_with_handedness(-self.handedness);

        assert_eq!(
            Axis::other(&self.main_face.axis, &self.side_face().axis).unwrap(),
            output
        );

        output
    }
}

pub const SOLVED: [CenterEdge; 24] = {
    use faces::*;
    use Direction::*;

    [
        CenterEdge::new(R, Positive, Positive),
        CenterEdge::new(R, Positive, Negative),
        CenterEdge::new(R, Negative, Negative),
        CenterEdge::new(R, Negative, Positive),
        CenterEdge::new(U, Positive, Positive),
        CenterEdge::new(U, Positive, Positive),
        CenterEdge::new(U, Positive, Negative),
        CenterEdge::new(U, Positive, Negative),
        CenterEdge::new(F, Negative, Negative),
        CenterEdge::new(F, Negative, Negative),
        CenterEdge::new(F, Negative, Positive),
        CenterEdge::new(F, Negative, Positive),
        CenterEdge::new(L, Positive, Positive),
        CenterEdge::new(L, Positive, Negative),
        CenterEdge::new(L, Negative, Negative),
        CenterEdge::new(L, Negative, Positive),
        CenterEdge::new(D, Positive, Positive),
        CenterEdge::new(D, Positive, Positive),
        CenterEdge::new(D, Positive, Negative),
        CenterEdge::new(D, Positive, Negative),
        CenterEdge::new(B, Negative, Negative),
        CenterEdge::new(B, Negative, Negative),
        CenterEdge::new(B, Negative, Positive),
        CenterEdge::new(B, Negative, Positive),
    ]
};
