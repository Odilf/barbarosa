//! Collection of heuristics for the A* search algorithm.

pub use manhattan::manhattan;
pub use mus::{mus, mus_with_fallback};

use crate::generic::Cube;

pub mod manhattan;
mod mus;

/// A heuristic that just always returns 0. It can be used to basically fall back to BFS.
pub fn zero(&_: &impl Cube) -> f32 {
    0.0
}
