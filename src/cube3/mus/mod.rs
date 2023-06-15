use super::{Corner, Edge};

pub mod cache;
pub mod deindex;
pub mod index;

type Corners = [Corner; 8];
type HalfEdges = [Edge; 6];
