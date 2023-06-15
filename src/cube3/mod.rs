//! 3x3x3 Rubik's cube implementation.

use std::hash::Hash;

use nalgebra::vector;

use self::moves::do_move;
pub use self::{
    moves::Move,
    piece::{Corner, Edge, Piece},
    space::{Axis, Direction},
};

mod piece;

pub mod heuristics;
pub mod moves;
pub mod random;
pub mod space;

mod invariants;
pub mod mus;
mod test;

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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube {
    /// The edges of the cube.
    pub edges: [Edge; 12],

    /// The corners of the cube.
    pub corners: [Corner; 8],
}

// TODO: Would be cool if this was replaced with a macro
const SOLVED_CUBE: Cube = {
    use Axis::*;
    use Direction::*;

    Cube {
        // Edges are set up this way so that an X2 rotation increases the index by 6
        edges: [
            Edge::oriented(X, vector![Positive, Positive]),
            Edge::oriented(X, vector![Positive, Negative]),
            Edge::oriented(Y, vector![Positive, Positive]),
            Edge::oriented(Y, vector![Positive, Negative]),
            Edge::oriented(Z, vector![Positive, Positive]),
            Edge::oriented(Z, vector![Negative, Positive]),
            Edge::oriented(X, vector![Negative, Negative]),
            Edge::oriented(X, vector![Negative, Positive]),
            Edge::oriented(Y, vector![Negative, Positive]),
            Edge::oriented(Y, vector![Negative, Negative]),
            Edge::oriented(Z, vector![Positive, Negative]),
            Edge::oriented(Z, vector![Negative, Negative]),
        ],
        corners: [
            Corner::oriented(vector![Positive, Positive, Positive]),
            Corner::oriented(vector![Positive, Positive, Negative]),
            Corner::oriented(vector![Positive, Negative, Positive]),
            Corner::oriented(vector![Positive, Negative, Negative]),
            Corner::oriented(vector![Negative, Positive, Positive]),
            Corner::oriented(vector![Negative, Positive, Negative]),
            Corner::oriented(vector![Negative, Negative, Positive]),
            Corner::oriented(vector![Negative, Negative, Negative]),
        ],
    }
};

impl Cube {
    /// A reference to a solved cube.
    pub const fn solved() -> &'static Self {
        &SOLVED_CUBE
    }

    /// A new solved cube.
    pub fn new_solved() -> Self {
        Self::solved().clone()
    }

    /// Determines if the cube is solved.
    pub fn is_solved(&self) -> bool {
        let solved = Self::solved();
        solved.edges == self.edges && solved.corners == self.corners
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new_solved()
    }
}

impl Cube {
    /// Applies a move to the cube.
    ///
    /// See also [Cube::into_move] for the owned version.
    pub fn do_move(&mut self, mov: &Move) {
        do_move(&mut self.edges, mov);
        do_move(&mut self.corners, mov);
    }

    /// Gets the moved cube.
    ///
    /// See also [Cube::do_move] for the mutable version.
    pub fn moved(self, mov: &Move) -> Self {
        let mut cube = self;
        cube.do_move(mov);
        cube
    }

    /// Applies an algorithm to the cube.
    pub fn apply_alg<'a>(&mut self, alg: impl Iterator<Item = &'a Move>) {
        alg.into_iter().for_each(|mov| self.do_move(mov));
    }
}

impl Hash for Cube {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.edges.hash(state);
        self.corners.hash(state);
    }
}

impl From<&Vec<Move>> for Cube {
    fn from(alg: &Vec<Move>) -> Self {
        let mut cube = Self::new_solved();
        cube.apply_alg(alg.iter());
        cube
    }
}
