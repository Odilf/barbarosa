use crate::{
    cube_n::invariants::fix_corner_multiplicity,
    cube3::{
        mus::{index::OrientationIndexable, Corners, HalfEdges},
        Corner, Cube3, Edge,
    },
    generic::Cube,
};

use super::{deindex_orientations, deindex_positions, Deindexable};

impl Deindexable for Corners {
    fn from_index(index: usize) -> Self {
        let position_index = index / Self::ORIENTATION_SET_SIZE;
        let orientation_index = index % Self::ORIENTATION_SET_SIZE;

        let position_indices = deindex_positions::<Self, 8, 8>(position_index);
        let orientations = deindex_orientations::<Corner, 7>(orientation_index);

        let mut corners = position_indices.map(|i| Cube3::solved().corners[i].clone());

        for (corner, orientation) in corners.iter_mut().zip(orientations.iter()) {
            for _ in 0..*orientation {
                corner.twist();
            }
        }

        fix_corner_multiplicity(&mut corners);

        corners
    }
}

impl Deindexable for HalfEdges {
    fn from_index(index: usize) -> Self {
        let position_index = index / Self::ORIENTATION_SET_SIZE;
        let orientation_index = index % Self::ORIENTATION_SET_SIZE;

        let position_indices = deindex_positions::<Self, 6, 12>(position_index);
        let orientations = deindex_orientations::<Edge, 6>(orientation_index);

        let mut edges = position_indices.map(|i| Cube3::solved().edges[i].clone());

        for (edge, orientation) in edges.iter_mut().zip(orientations.iter()) {
            if *orientation != 0 {
                edge.flip();
            }
        }

        edges
    }
}
