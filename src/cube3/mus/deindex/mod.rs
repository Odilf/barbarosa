mod test;

use crate::cube3::{moves::alg::perm, Corner, Cube, Edge};

use super::index::{
    disposition_multipliers, CubeIndices, Indexable, OrientationIndexable, PositionIndexable,
};

pub trait Deindexable: Indexable {
    fn from_index(index: usize) -> Self;
}

impl Deindexable for [Corner; 8] {
    fn from_index(index: usize) -> Self {
        let position_index = index / Self::ORIENTATION_SET_SIZE;
        let orientation_index = index % Self::ORIENTATION_SET_SIZE;

        let position_indices = deindex_positions::<Self, 8, 8>(position_index);
        let orientations = deindex_orientations::<Self, 8>(orientation_index);

        let mut corners = position_indices.map(|i| Cube::solved().corners[i].clone());

        for (corner, orientation) in corners.iter_mut().zip(orientations.iter()) {
            for _ in 0..*orientation {
                corner.twist();
            }
        }

        corners
    }
}

impl Deindexable for [Edge; 6] {
    fn from_index(index: usize) -> Self {
        let position_index = index / Self::ORIENTATION_SET_SIZE;
        let orientation_index = index % Self::ORIENTATION_SET_SIZE;

        let position_indices = deindex_positions::<Self, 6, 12>(position_index);
        let orientations = deindex_orientations::<Self, 6>(orientation_index);

        let mut edges = position_indices.map(|i| Cube::solved().edges[i].clone());

        for (edge, orientation) in edges.iter_mut().zip(orientations.iter()) {
            if *orientation != 0 {
                edge.flip();
            }
        }

        edges
    }
}

/// Takes the index of the orientation and returns the indices of the orientations
fn deindex_orientations<T: OrientationIndexable, const N: usize>(index: usize) -> [usize; N] {
    let mut output = [0; N];
    let mut remaining_index = index;

    for elem in output.iter_mut().rev() {
        *elem = remaining_index % T::ORIENTATION_SET_SIZE;
        remaining_index /= T::ORIENTATION_SET_SIZE;
    }

    output
}

/// Takes the index of the position and returns the indices of the positions
fn deindex_choices<T: PositionIndexable, const N: usize, const T_POSITION_SET_SIZE: usize>(
    index: usize,
) -> [usize; N] {
    let multipliers = disposition_multipliers::<T, N, T_POSITION_SET_SIZE>();
    let mut choices = [0; N];
    let mut remaining_index = index;

    for (choice, multiplier) in choices.iter_mut().zip(multipliers.iter()) {
        *choice = remaining_index / multiplier;
        remaining_index %= multiplier;
    }

    choices
}

fn choices_to_permutations<const N: usize, const T_POSITION_SET_SIZE: usize>(
    choices: [usize; N],
) -> [usize; N] {
    let mut permutations = [0; N];
    let mut used = [false; T_POSITION_SET_SIZE];

    for (choice, perm_index) in choices.iter().zip(permutations.iter_mut()) {
		// dbg!(used, choice, &perm_index);

        *perm_index = used
            .iter()
            .enumerate()
            .filter(|(_, used)| !**used)
            .nth(*choice)
            .unwrap()
            .0;

		used[*perm_index] = true;
    }

    permutations
}

fn deindex_positions<T: PositionIndexable, const N: usize, const T_POSITION_SET_SIZE: usize>(
    index: usize,
) -> [usize; N] {
    let choices = deindex_choices::<T, N, T_POSITION_SET_SIZE>(index);
    choices_to_permutations::<N, T_POSITION_SET_SIZE>(choices)
}

impl Cube {
    pub fn from_indices(indices: CubeIndices) -> Self {
        let corners = <[Corner; 8]>::from_index(indices.corners);
        let edges = indices
            .edges
            .map(|edge_index| <[Edge; 6]>::from_index(edge_index));

        let edges = edges
            .concat()
            .try_into()
            .expect("2 arrays of length 6 make 12 in total");

        Cube { corners, edges }
    }
}
