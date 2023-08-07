mod set;

use std::fmt::Debug;

pub use set::PieceSet;

/// A piece on the cube. It is generic over a const `SET_SIZE` which is the number of
/// distinct positions that the piece can be in. You also need to define a position, such
/// that it is impossible to have two pieces in the same position.
///
/// The original position of a piece is the way that it gets identified, you can think of it
/// as the color information of a piece. For example, the RUF corner is the one that has the
/// original position of `[1, 1, 1]`. Each piece has a reference of these positions, which makes
/// it so that we only need to store the current position instead of the original and the current.
pub trait Piece<const SET_SIZE: usize>: Sized + PartialEq + Clone + Debug {
    /// The solved set of pieces.
    const SOLVED: [Self; SET_SIZE];

    /// The position type of the piece. It should be a type with exactly `SET_SIZE` different possible values.
    type Position: PartialEq + Debug;

    /// The reference positions of the piece. This is used to define that, in an array of pieces, the piece at
    /// index `i` was originally at position `REFERENCE_POSITIONS[i]`.
    const REFERENCE_POSITIONS: [Self::Position; SET_SIZE];

    /// Returns the current position of the piece
    fn position(&self) -> Self::Position;

    /// Determines whether the piece is solved, given its original position.
    fn is_solved(&self, original_pos: &Self::Position) -> bool;
}
