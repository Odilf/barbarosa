#![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]

//! Barbarosa is a rust library for interacting with rubik's cubes. It is
//! designed to be fast.
//!
//! Currently only the 3x3x3 cube is supported.

pub mod cube3;
pub mod search;
