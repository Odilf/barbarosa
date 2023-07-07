//! Collection of heuristics for the A* search algorithm.

pub use manhattan::manhattan;
pub use mus::{mus, mus_with_fallback};

mod manhattan;
mod mus;
