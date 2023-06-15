use crate::cube3::mus::index::OrientationIndexable;
use crate::cube3::mus::{Corners, HalfEdges};
use crate::cube3::{Corner, Cube, Edge};

use super::{deindex_orientations, deindex_positions, Deindexable};

impl Deindexable for Corners {
    fn from_index(index: usize) -> Self {
        let position_index = index / Self::ORIENTATION_SET_SIZE;
        let orientation_index = index % Self::ORIENTATION_SET_SIZE;

        let position_indices = deindex_positions::<Self, 8, 8>(position_index);
        let orientations = deindex_orientations::<Corner, 7>(orientation_index);

        let mut corners = position_indices.map(|i| Cube::solved().corners[i].clone());

        for (corner, orientation) in corners.iter_mut().zip(orientations.iter()) {
            for _ in 0..*orientation {
                corner.twist();
            }
        }

        // TODO: This is tehcnially repeated from `Cube::random()`
        let oriented_corners: i32 = corners
            .iter()
            .map(|corner| corner.orientation_index() as i32)
            .sum();
        let corner_orientation_offset = oriented_corners % 3;
        for _ in 0..corner_orientation_offset {
            corners[7].twist();
        }

        corners
    }
}

impl Deindexable for HalfEdges {
    fn from_index(index: usize) -> Self {
        let position_index = index / Self::ORIENTATION_SET_SIZE;
        let orientation_index = index % Self::ORIENTATION_SET_SIZE;

        let position_indices = deindex_positions::<Self, 6, 12>(position_index);
        let orientations = deindex_orientations::<Edge, 6>(orientation_index);

        let mut edges = position_indices.map(|i| Cube::solved().edges[i].clone());

        for (edge, orientation) in edges.iter_mut().zip(orientations.iter()) {
            if *orientation != 0 {
                edge.flip();
            }
        }

        edges
    }
}
