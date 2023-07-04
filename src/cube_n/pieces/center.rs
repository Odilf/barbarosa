use nalgebra::vector;

use crate::cube_n::{
    moves::rotation::{AxisRotation, Rotatable},
    space::{Axis, Direction},
    WideAxisMove,
};

use super::Corner;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Center<T: Rotatable>(pub T);

impl<T: Rotatable> Rotatable for Center<T> {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.0.rotate(rotation);
    }
}

pub(crate) fn corner_in_wide_move<const N: u32>(
    center: &Center<Corner>,
    piece_depth: u32,
    m: &WideAxisMove<N>,
) -> bool {
    let corner = &center.0;

    // TODO: This might be wrong
    // If on the same direction
    if corner.position[m.face().axis] == m.face().direction {
        if corner.orientation_axis == m.face().axis {
            return true;
        }

        if piece_depth <= m.depth() {
            return true;
        }
    }

    false
}

pub const SOLVED_CORNERS: [Center<Corner>; 24] = {
    use Direction::*;

    [
        Center(Corner::oriented(vector![Positive, Positive, Positive])),
        Center(Corner::oriented(vector![Positive, Positive, Negative])),
        Center(Corner::oriented(vector![Positive, Negative, Positive])),
        Center(Corner::oriented(vector![Positive, Negative, Negative])),
        Center(Corner::oriented(vector![Negative, Positive, Positive])),
        Center(Corner::oriented(vector![Negative, Positive, Negative])),
        Center(Corner::oriented(vector![Negative, Negative, Positive])),
        Center(Corner::oriented(vector![Negative, Negative, Negative])),
        Center(Corner::new(vector![Positive, Positive, Positive], Axis::Y)),
        Center(Corner::new(vector![Positive, Positive, Negative], Axis::Y)),
        Center(Corner::new(vector![Positive, Negative, Positive], Axis::Y)),
        Center(Corner::new(vector![Positive, Negative, Negative], Axis::Y)),
        Center(Corner::new(vector![Negative, Positive, Positive], Axis::Y)),
        Center(Corner::new(vector![Negative, Positive, Negative], Axis::Y)),
        Center(Corner::new(vector![Negative, Negative, Positive], Axis::Y)),
        Center(Corner::new(vector![Negative, Negative, Negative], Axis::Y)),
        Center(Corner::new(vector![Positive, Positive, Positive], Axis::Z)),
        Center(Corner::new(vector![Positive, Positive, Negative], Axis::Z)),
        Center(Corner::new(vector![Positive, Negative, Positive], Axis::Z)),
        Center(Corner::new(vector![Positive, Negative, Negative], Axis::Z)),
        Center(Corner::new(vector![Negative, Positive, Positive], Axis::Z)),
        Center(Corner::new(vector![Negative, Positive, Negative], Axis::Z)),
        Center(Corner::new(vector![Negative, Negative, Positive], Axis::Z)),
        Center(Corner::new(vector![Negative, Negative, Negative], Axis::Z)),
    ]
};
