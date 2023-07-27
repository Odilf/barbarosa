//! The 3x3x3 Rubik's cube.
//!
//! See [Cube3] and [cube_n](crate::cube_n) for more information.

pub mod heuristics;
pub mod mus;

mod test;

use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom};

use crate::{
    cube_n::{pieces, space::Axis},
    generic::{self, moves::AsMove, Cube},
};

use super::invariants::{fix_corner_multiplicity, fix_edge_flip_parity, fix_swap_parity};

use super::{AxisMove, Corner, Edge};

/// A 3x3x3 Rubik's cube.
///
/// The cube is represented by 12 [Edge] pieces and 8 [Corner] pieces.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Cube3 {
    /// The edges of the cube.
    pub edges: [Edge; 12],

    /// The corners of the cube.
    pub corners: [Corner; 8],
}

const SOLVED_CUBE: Cube3 = Cube3 {
    edges: pieces::edge::SOLVED,
    corners: pieces::corner::SOLVED,
};

impl generic::Cube for Cube3 {
    fn solved() -> &'static Self {
        &SOLVED_CUBE
    }
}

impl AsMove for Cube3 {
    type Move = AxisMove;
}

impl generic::Movable<AxisMove> for Cube3 {
    fn apply(&mut self, m: &AxisMove) {
        self.corners.apply(m);
        self.edges.apply(m);
    }
}

impl Distribution<Cube3> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Cube3 {
        let mut cube = Cube3::new_solved();

        // Move pieces
        cube.edges.shuffle(rng);
        cube.corners.shuffle(rng);

        // Flip pieces
        cube.edges
            .iter_mut()
            .for_each(|edge| edge.oriented = rng.gen());
        cube.corners
            .iter_mut()
            .for_each(|corner| corner.orientation_axis = rng.gen());

        fix_swap_parity(&mut cube);

        fix_edge_flip_parity(&mut cube.edges);
        fix_corner_multiplicity(&mut cube.corners);

        cube
    }
}
