use nalgebra::{vector, Vector3};

use crate::{
    cube_n::{
        moves::rotation::{AxisRotation, Rotatable},
        space::{Axis, Direction},
        WideAxisMove,
    },
    generic,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CenterCorner {
    position: Vector3<Direction>,
    axis: Axis,
}

impl generic::Piece for CenterCorner {}

impl Rotatable for CenterCorner {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.position.rotate(rotation);
        self.axis.rotate(rotation);
    }
}

impl CenterCorner {
    const fn new(position: Vector3<Direction>, axis: Axis) -> Self {
        Self { position, axis }
    }

    pub fn is_solved(&self, original: &CenterCorner) -> bool {
        self.position[self.axis] == original.position[original.axis]
    }

    pub fn in_wide_move<const N: u32>(&self, piece_depth: u32, m: &WideAxisMove<N>) -> bool {
        // If on the same direction
        if self.position[m.face().axis] == m.face().direction {
            if piece_depth <= m.depth() {
                return true;
            }

            if self.axis == m.face().axis {
                return true;
            }
        }

        false
    }
}

pub const SOLVED: [CenterCorner; 24] = {
    use Direction::*;

    [
        CenterCorner::new(vector![Positive, Positive, Positive], Axis::X),
        CenterCorner::new(vector![Positive, Positive, Negative], Axis::X),
        CenterCorner::new(vector![Positive, Negative, Positive], Axis::X),
        CenterCorner::new(vector![Positive, Negative, Negative], Axis::X),
        CenterCorner::new(vector![Negative, Positive, Positive], Axis::X),
        CenterCorner::new(vector![Negative, Positive, Negative], Axis::X),
        CenterCorner::new(vector![Negative, Negative, Positive], Axis::X),
        CenterCorner::new(vector![Negative, Negative, Negative], Axis::X),
        CenterCorner::new(vector![Positive, Positive, Positive], Axis::Y),
        CenterCorner::new(vector![Positive, Positive, Negative], Axis::Y),
        CenterCorner::new(vector![Positive, Negative, Positive], Axis::Y),
        CenterCorner::new(vector![Positive, Negative, Negative], Axis::Y),
        CenterCorner::new(vector![Negative, Positive, Positive], Axis::Y),
        CenterCorner::new(vector![Negative, Positive, Negative], Axis::Y),
        CenterCorner::new(vector![Negative, Negative, Positive], Axis::Y),
        CenterCorner::new(vector![Negative, Negative, Negative], Axis::Y),
        CenterCorner::new(vector![Positive, Positive, Positive], Axis::Z),
        CenterCorner::new(vector![Positive, Positive, Negative], Axis::Z),
        CenterCorner::new(vector![Positive, Negative, Positive], Axis::Z),
        CenterCorner::new(vector![Positive, Negative, Negative], Axis::Z),
        CenterCorner::new(vector![Negative, Positive, Positive], Axis::Z),
        CenterCorner::new(vector![Negative, Positive, Negative], Axis::Z),
        CenterCorner::new(vector![Negative, Negative, Positive], Axis::Z),
        CenterCorner::new(vector![Negative, Negative, Negative], Axis::Z),
    ]
};
