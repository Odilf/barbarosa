use crate::cube3::{Corner, Cube, Edge};

use super::{
    factorial, orientation_permutation_index, position_disposition_index, OrientationIndexable,
    PositionIndexable,
};

impl PositionIndexable for Corner {
    fn position_index(&self) -> usize {
        Cube::solved()
            .corners
            .iter()
            .position(|corner| corner.position == self.position)
            .unwrap()
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
        Cube::solved()
            .edges
            .iter()
            .position(|edge| edge.normal_axis == self.normal_axis && edge.position == self.position)
            .unwrap()
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




impl PositionIndexable for [Edge; 6] {
    fn position_index(&self) -> usize {
        position_disposition_index::<_, 6, { Edge::POSITION_SET_SIZE }>(self)
    }

    const POSITION_SET_SIZE: usize = factorial(12) / factorial(12 - 6);
}

impl OrientationIndexable for [Edge; 6] {
    fn orientation_index(&self) -> usize {
        orientation_permutation_index(self)
    }

    const ORIENTATION_SET_SIZE: usize = 2usize.pow(6);
}




impl PositionIndexable for [Corner; 8] {
    fn position_index(&self) -> usize {
        position_disposition_index::<_, 8, { Corner::POSITION_SET_SIZE }>(self)
    }

    const POSITION_SET_SIZE: usize = factorial(8);
}

impl OrientationIndexable for [Corner; 8] {
    fn orientation_index(&self) -> usize {
        // The last corner is determined by the other 7 so we should ignore it
        let useful_corners: &[Corner; 7] = self[0..7].try_into().unwrap();
        orientation_permutation_index(useful_corners)
    }

    const ORIENTATION_SET_SIZE: usize = 3usize.pow(7);
}
