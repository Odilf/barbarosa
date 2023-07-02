//! NxNxNs cube implementation.

pub mod cube2;
pub mod cube3;
pub mod cube4;

pub mod invariants;
pub mod moves;
pub mod pieces;
pub mod space;

pub use moves::{AxisMove, WideAxisMove};
pub use pieces::{Corner, Edge, Wing};

pub use cube2::Cube2;
pub use cube3::Cube3;
