//! Traits and structs for pieces on the cube.

mod set;

use std::fmt::Debug;

pub use set::{PieceSet, PieceSetDescriptor};

/// A piece on the cube.
///
/// A piece is identified by its original position, you can think of it
/// as the color information of a piece. For example, the RUF corner is the one that has the
/// original position of `[1, 1, 1]`. Each piece has a reference of these positions, which makes
/// it so that we only need to store the current position instead of the original and the current.
pub trait Piece: Sized + PartialEq + Clone + Debug {
    /// The position type of the piece.
    ///
    /// Positions are unique in [`PieceSet`]s. That is, there can only
    /// be one piece at each position.
    ///
    /// It should have finite different possible values, such that you
    /// can implement [`PieceSetDescriptor`] for it with a specific `N`.
    type Position: PartialEq + Debug;

    /// Returns the current position of the piece
    fn position(&self) -> Self::Position;

    /// Determines whether the piece is solved, given its original position.
    fn is_solved(&self, original_pos: &Self::Position) -> bool;
}
