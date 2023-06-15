mod implementations;
mod test;

use crate::cube3::Cube;

use super::{
    index::{
        disposition_multipliers, CubeIndices, Indexable, OrientationIndexable, PositionIndexable,
    },
    Corners, HalfEdges,
};

pub trait Deindexable: Indexable {
    fn from_index(index: usize) -> Self;
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
        let corners = Corners::from_index(indices.corners);
        let edges = indices
            .edges
            .map(|edge_index| HalfEdges::from_index(edge_index));

        let edges = edges
            .concat()
            .try_into()
            .expect("2 arrays of length 6 make 12 in total");

        Cube { corners, edges }
    }
}
