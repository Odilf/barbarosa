#![warn(missing_docs)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

pub mod cube_n;
pub mod generic;
pub mod prelude;

pub use cube_n::cube3;
