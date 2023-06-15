use nalgebra::Vector3;

use super::{moves::Rotation, space::Face};

mod corner;
mod edge;

pub use corner::Corner;
pub use edge::Edge;

/// An enum that represents either a corner or a face.
///
/// This is useful for when you want to iterate over all pieces without having to use
/// dinamic dispatch or caring about the trait being object safe and what not.
// #[enum_dispatch]
// pub enum PieceEnum {
//     /// The corner piece variant. Visit [Corner] for more information.
//     Corner,
//     /// The edge piece variant. Visit [Edge] for more information.
//     Edge,
// }

/// A trait that represents a piece of the cube.
///
/// [Corner] and [Edge] implement this trait.
// #[enum_dispatch(PieceEnum)]
pub trait Piece {
    type Position;

    /// The position of the piece, relative to the center of the cube.
    fn coordinates(&self) -> Vector3<i8>;

    /// A piece only rotates. Moves are part of the cube, since
    /// they're just rotations but only to a subset of the pieces.
    fn rotate(&mut self, rotation: &Rotation);

    /// Returns `true` if the piece is on the given face.
    fn in_face(&self, face: &Face) -> bool;
}
