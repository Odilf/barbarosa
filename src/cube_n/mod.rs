//! NxNxNs cube implementation.

pub mod cube3;

pub mod moves;
pub mod pieces;
pub mod space;

pub use moves::AxisMove;
pub use pieces::{Corner, Edge};

pub use cube3::Cube3;
