//! Pieces of an NxNxN cube.

pub mod center;
pub mod corner;
pub mod edge;
pub mod wing;

pub use center::corner::CornerCenter;
pub use corner::Corner;
pub use edge::Edge;
pub use wing::Wing;
