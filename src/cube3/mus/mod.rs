use super::{Corner, Edge};

pub mod cache;
mod deindex;
mod index;

type Corners = [Corner; 8];
type HalfEdges = [Edge; 6];
