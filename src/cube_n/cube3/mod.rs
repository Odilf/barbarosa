//! The 3x3x3 Rubik's cube.
//!
//! See [Cube3] for more information. Also [crate::cube_n] and [crate::generic] for more generic aspects
//! about the 3x3, such as moving and pieces lol.

pub mod heuristics;
pub mod mus;
mod test;

use rand::seq::SliceRandom;

use crate::{
    cube_n::{pieces, space::Axis},
    generic::{self, Alg},
};

use super::invariants::{fix_corner_multiplicity, fix_edge_flip_parity, fix_swap_parity};

use super::{AxisMove, Corner, Edge};

/// A 3x3x3 Rubik's cube.
///
/// The cube is represented by 12 [Edge] pieces and 8 [Corner] pieces.
///
/// # Piece position
/// A piece only stores where it is, not what it is. That is, you couldn't tell
/// the color of, for example, a corner just by the information in the [Corner] struct.
///
/// Rather, the cube is responsible for keeping track for which piece is which. Simply,
/// the "color" of a piece is determined by that position in [Cube::solved()]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Cube3 {
    /// The edges of the cube.
    pub edges: [Edge; 12],

    /// The corners of the cube.
    pub corners: [Corner; 8],
}

// TODO: Would be cool if this was replaced with a macro
const SOLVED_CUBE: Cube3 = Cube3 {
    edges: pieces::edge::SOLVED,
    corners: pieces::corner::SOLVED,
};

impl generic::Cube for Cube3 {
    fn solved() -> &'static Self {
        &SOLVED_CUBE
    }

    fn random_with_rng(rng: &mut impl rand::Rng) -> Self {
        let mut cube = Self::new_solved();

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

    type Move = AxisMove;
    type Alg = Alg<AxisMove>;
}

impl generic::Movable<AxisMove> for Cube3 {
    fn apply(&mut self, m: &AxisMove) {
        self.corners.apply(m);
        self.edges.apply(m);
    }
}
