use crate::{
    cube3::{
        mus::{CornersMUS, HalfEdgesMUS},
        Corner, Edge,
    },
    generic::Piece,
};

use super::{
    factorial, orientation_permutation_index, position_disposition_index, OrientationIndexable,
    PositionIndexable,
};

impl PositionIndexable for Corner {
    fn position_index(&self) -> usize {
        Corner::REFERENCE_POSITIONS
            .iter()
            .position(|reference_pos| *reference_pos == self.position)
            .expect("There should be a corner in every position")
    }

    const POSITION_SET_SIZE: usize = 8;
}

impl OrientationIndexable for Corner {
    fn orientation_index(&self) -> usize {
        self.orientation_index()
    }

    const ORIENTATION_SET_SIZE: usize = 3;
}

impl PositionIndexable for Edge {
    fn position_index(&self) -> usize {
        Edge::REFERENCE_POSITIONS
            .iter()
            .position(|reference_pos| *reference_pos == self.position())
            .expect("There should be an edge in every position")
    }

    const POSITION_SET_SIZE: usize = 12;
}

impl OrientationIndexable for Edge {
    fn orientation_index(&self) -> usize {
        match self.oriented {
            true => 0,
            false => 1,
        }
    }

    const ORIENTATION_SET_SIZE: usize = 2;
}

impl PositionIndexable for HalfEdgesMUS {
    fn position_index(&self) -> usize {
        position_disposition_index::<_, 6, { Edge::POSITION_SET_SIZE }>(self)
    }

    const POSITION_SET_SIZE: usize = factorial(12) / factorial(12 - 6);
}

impl OrientationIndexable for HalfEdgesMUS {
    fn orientation_index(&self) -> usize {
        orientation_permutation_index(self)
    }

    const ORIENTATION_SET_SIZE: usize = 2usize.pow(6);
}

impl PositionIndexable for CornersMUS {
    fn position_index(&self) -> usize {
        position_disposition_index::<_, 8, { Corner::POSITION_SET_SIZE }>(self)
    }

    const POSITION_SET_SIZE: usize = factorial(8);
}

impl OrientationIndexable for CornersMUS {
    fn orientation_index(&self) -> usize {
        // The last corner is determined by the other 7 so we should ignore it
        let useful_corners: &[Corner; 7] = self[0..7].try_into().unwrap();
        orientation_permutation_index(useful_corners)
    }

    const ORIENTATION_SET_SIZE: usize = 3usize.pow(7);
}
