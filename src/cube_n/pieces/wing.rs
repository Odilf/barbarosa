//! The wing pieces for 4x4+. See [Wing] for more info.

use nalgebra::vector;

use crate::{
    cube_n::{
        moves::rotation::{AxisRotation, Rotatable},
        space::{Axis, Direction},
        WideAxisMove,
    },
    generic,
};

use super::Edge;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wing {
    // TODO: This is very bodgy lol
    corresponding_edge: Edge,
}

impl generic::Piece for Wing {}

impl Rotatable for Wing {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.corresponding_edge.rotate(rotation);
    }
}

pub fn in_wide_move<const N: u32>(wing: &Wing, wing_depth: u32, m: &WideAxisMove<N>) -> bool {
    // If just on the same face
    if m.axis_move.face.contains_edge(&wing.corresponding_edge) {
        return true;
    }

    // If on parallel slices (so, same normal)
    if wing.corresponding_edge.normal_axis == m.face().axis {
        // If it's on the right depth
        if wing_depth == m.depth() {
            // If it's on the right side
            // TODO: Is this actually right? I don't think it is
            let should_be_oriented = m.face().direction == Direction::Positive;
            if wing.corresponding_edge.oriented == should_be_oriented {
                return true;
            }
        }
    }

    return false;
}

impl Wing {
    pub const fn new(corresponding_edge: Edge) -> Self {
        Self { corresponding_edge }
    }
}

pub const SOLVED: [Wing; 24] = {
    use Axis::*;
    use Direction::*;

    [
        Wing::new(Edge::oriented(X, vector![Positive, Positive])),
        Wing::new(Edge::oriented(X, vector![Positive, Negative])),
        Wing::new(Edge::oriented(Y, vector![Positive, Positive])),
        Wing::new(Edge::oriented(Y, vector![Positive, Negative])),
        Wing::new(Edge::oriented(Z, vector![Positive, Positive])),
        Wing::new(Edge::oriented(Z, vector![Negative, Positive])),
        Wing::new(Edge::oriented(X, vector![Negative, Negative])),
        Wing::new(Edge::oriented(X, vector![Negative, Positive])),
        Wing::new(Edge::oriented(Y, vector![Negative, Positive])),
        Wing::new(Edge::oriented(Y, vector![Negative, Negative])),
        Wing::new(Edge::oriented(Z, vector![Positive, Negative])),
        Wing::new(Edge::oriented(Z, vector![Negative, Negative])),
        Wing::new(Edge::oriented(X, vector![Positive, Positive]).flipped()),
        Wing::new(Edge::oriented(X, vector![Positive, Negative]).flipped()),
        Wing::new(Edge::oriented(Y, vector![Positive, Positive]).flipped()),
        Wing::new(Edge::oriented(Y, vector![Positive, Negative]).flipped()),
        Wing::new(Edge::oriented(Z, vector![Positive, Positive]).flipped()),
        Wing::new(Edge::oriented(Z, vector![Negative, Positive]).flipped()),
        Wing::new(Edge::oriented(X, vector![Negative, Negative]).flipped()),
        Wing::new(Edge::oriented(X, vector![Negative, Positive]).flipped()),
        Wing::new(Edge::oriented(Y, vector![Negative, Positive]).flipped()),
        Wing::new(Edge::oriented(Y, vector![Negative, Negative]).flipped()),
        Wing::new(Edge::oriented(Z, vector![Positive, Negative]).flipped()),
        Wing::new(Edge::oriented(Z, vector![Negative, Negative]).flipped()),
    ]
};
