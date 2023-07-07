//! # The "moves until solved" (MUS) cache
//!
//! This module provides all the functionality for generating and using the MUS cache.
//!
//! The MUS cache can be used as a pretty good heuristic for solving the 3x3x3 cube.
//!
//! ## How it works
//!
//! The MUS cache provides two lookup tables: one for the corners and one for sets of 6 edges.
//! It would be nice to cache every possible state, but this is totally unfeasible since the
//! 3x3x3 has `(12! * 8! / 2) * (3^7 * 2^11) = 43,252,003,274,489,856,000` states. Since it is
//! know that every configuration can be solved in no more than 20 moves, we can store each state
//! in a bytes. However that's still 43 *million* **petabatyes** of storage. so yeah
//!
//! However, we can greatly reduce the problem space by only considering corners and sets of 6 edges
//! at a time, which have 88 and 42 million different states respectively. 42mb and 88mb is less than
//! what most electron apps use.
//!
//! In the heuristic, we retrieve the number of moves until solved for each of the 8 corners and each
//! of the two sets of 6 edges and choose the maximum.
//!
//! ## Generation
//!
//! todo!()

use super::{Corner, Edge};

pub mod cache;
pub mod deindex;
pub mod index;

type Corners = [Corner; 8];
type HalfEdges = [Edge; 6];
