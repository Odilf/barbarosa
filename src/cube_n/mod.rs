//! NxNxNs cube implementation.

pub mod cube2;
pub mod cube3;

pub mod moves;
pub mod pieces;
pub mod space;
pub mod invariants;

pub use moves::AxisMove;
pub use pieces::{Corner, Edge};

pub use cube3::Cube3;
pub use cube2::Cube2;
