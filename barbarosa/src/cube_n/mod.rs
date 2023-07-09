//! NxNxNs cube implementation.

pub mod cube2;
pub mod cube3;
pub mod cube4;
pub mod cube5;
pub mod cube6;
pub mod cube7;

pub mod invariants;
pub mod moves;
pub mod pieces;
pub mod search;
pub mod space;

pub use moves::{AxisMove, WideAxisMove};
pub use pieces::{Corner, Edge, Wing};

pub use cube2::Cube2;
pub use cube3::Cube3;
pub use cube4::Cube4;
pub use cube5::Cube5;
pub use cube6::Cube6;
pub use cube7::Cube7;
