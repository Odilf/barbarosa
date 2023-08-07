use crate::{
    cube3::{
        mus::{index::OrientationIndexable, CornersMUS, HalfEdgesMUS},
        Corner, Cube3, Edge,
    },
    cube_n::{invariants::fix_corner_multiplicity, pieces::corner::CornerSet},
    generic::Cube,
};

use super::{deindex_orientations, deindex_positions, Deindexable};

impl Deindexable for CornersMUS {
    fn from_index(index: usize) -> Self {
        let position_index = index / Self::ORIENTATION_SET_SIZE;
        let orientation_index = index % Self::ORIENTATION_SET_SIZE;

        let position_indices = deindex_positions::<Self, 8, 8>(position_index);
        let orientations = deindex_orientations::<Corner, 7>(orientation_index);

        let mut corners = position_indices.map(|i| Cube3::solved().corners.pieces()[i].clone());

        for (corner, orientation) in corners.iter_mut().zip(orientations.iter()) {
            for _ in 0..*orientation {
                corner.twist();
            }
        }

        let mut corner_set =
            CornerSet::new(corners).expect("Corners from index should always be valid");

        fix_corner_multiplicity(&mut corner_set);

        // TODO: This is slighlty inefficient, since we're creating the actual set, checking for correctness,
        // then taking the underlying data, cloning it, and repeating it on the next step. This should be ideally only 1 step
        corner_set.pieces().clone()
    }
}

impl Deindexable for HalfEdgesMUS {
    fn from_index(index: usize) -> Self {
        let position_index = index / Self::ORIENTATION_SET_SIZE;
        let orientation_index = index % Self::ORIENTATION_SET_SIZE;

        let position_indices = deindex_positions::<Self, 6, 12>(position_index);
        let orientations = deindex_orientations::<Edge, 6>(orientation_index);

        let mut edges = position_indices.map(|i| Cube3::solved().edges.pieces()[i].clone());

        for (edge, orientation) in edges.iter_mut().zip(orientations.iter()) {
            if *orientation != 0 {
                edge.flip();
            }
        }

        edges
    }
}
