use nalgebra::{vector, Vector3};

use crate::cube_n::{
    moves::rotation::{AxisRotation, Rotatable},
    space::{Axis, Direction},
    WideAxisMove,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CornerCenter {
    position: Vector3<Direction>,
    axis: Axis,
}

impl Rotatable for CornerCenter {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.position.rotate(rotation);
        self.axis.rotate(rotation);
    }
}

pub fn in_wide_move<const N: u32>(
    center: &CornerCenter,
    piece_depth: u32,
    m: &WideAxisMove<N>,
) -> bool {
    // If on the same direction
    if center.position[m.face().axis] == m.face().direction {
        if piece_depth <= m.depth() {
            return true;
        }

        if center.axis == m.face().axis {
            return true;
        }
    }

    false
}

impl CornerCenter {
    const fn new(position: Vector3<Direction>, axis: Axis) -> Self {
        Self { position, axis }
    }

    pub fn is_solved(&self, original: &CornerCenter) -> bool {
        self.position[self.axis] == original.position[original.axis]
    }
}

pub const SOLVED: [CornerCenter; 24] = {
    use Direction::*;

    [
        CornerCenter::new(vector![Positive, Positive, Positive], Axis::X),
        CornerCenter::new(vector![Positive, Positive, Negative], Axis::X),
        CornerCenter::new(vector![Positive, Negative, Positive], Axis::X),
        CornerCenter::new(vector![Positive, Negative, Negative], Axis::X),
        CornerCenter::new(vector![Negative, Positive, Positive], Axis::X),
        CornerCenter::new(vector![Negative, Positive, Negative], Axis::X),
        CornerCenter::new(vector![Negative, Negative, Positive], Axis::X),
        CornerCenter::new(vector![Negative, Negative, Negative], Axis::X),
        CornerCenter::new(vector![Positive, Positive, Positive], Axis::Y),
        CornerCenter::new(vector![Positive, Positive, Negative], Axis::Y),
        CornerCenter::new(vector![Positive, Negative, Positive], Axis::Y),
        CornerCenter::new(vector![Positive, Negative, Negative], Axis::Y),
        CornerCenter::new(vector![Negative, Positive, Positive], Axis::Y),
        CornerCenter::new(vector![Negative, Positive, Negative], Axis::Y),
        CornerCenter::new(vector![Negative, Negative, Positive], Axis::Y),
        CornerCenter::new(vector![Negative, Negative, Negative], Axis::Y),
        CornerCenter::new(vector![Positive, Positive, Positive], Axis::Z),
        CornerCenter::new(vector![Positive, Positive, Negative], Axis::Z),
        CornerCenter::new(vector![Positive, Negative, Positive], Axis::Z),
        CornerCenter::new(vector![Positive, Negative, Negative], Axis::Z),
        CornerCenter::new(vector![Negative, Positive, Positive], Axis::Z),
        CornerCenter::new(vector![Negative, Positive, Negative], Axis::Z),
        CornerCenter::new(vector![Negative, Negative, Positive], Axis::Z),
        CornerCenter::new(vector![Negative, Negative, Negative], Axis::Z),
    ]
};
