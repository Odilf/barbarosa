//! NxNxNs cube implementation.
//!
//! See [crate::generic] for more aspects that are generic across all cubes, such as moves and pieces.
//!
//! # Piece position
//!
//! todo!("This is totally outdated because of piece sets")
//!
//! A piece only stores where it is, not what it is. That is, you couldn't tell
//! the color of, for example, a corner just by the information in the [Corner] struct.
//!
//! Rather, the cube is responsible for keeping track for which piece is which. Simply,
//! the "color" of a piece is determined by that position in [Cube::solved](crate::generic::Cube::solved).
//!
//! You can use the functions [utils::item_at](crate::generic::utils::item_at) and
//! [utils::position_of_item](crate::generic::utils::position_of_item) to find where
//! pieces are.

mod cube2;
pub mod cube3;
mod cube4;
mod cube5;
mod cube6;
mod cube7;

pub mod invariants;
pub mod moves;
mod orientable;
pub mod pieces;
pub mod search;
pub mod space;

pub use moves::{AxisMove, WideAxisMove};
pub use orientable::{Orientable, Orientation};
pub use pieces::{center, Corner, Edge, Wing};

pub use cube2::Cube2;
pub use cube3::Cube3;
pub use cube4::Cube4;
pub use cube5::Cube5;
pub use cube6::Cube6;
pub use cube7::Cube7;

use crate::generic;

/// An NxNxN cube.
///
/// Currently just a marker trait.
pub trait CubeN: generic::Cube {
    /// The size of the cube.
    const N: u32;
}
