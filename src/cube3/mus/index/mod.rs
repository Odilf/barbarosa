use crate::cube3::{
    moves::{do_move, Amount, Rotation},
    Axis, Cube, Edge,
};

use super::{deindex::Deindexable, HalfEdges};

pub const fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        2..=20 => factorial(n - 1) * n,
        _ => 0,
    }
}

mod implementations;
mod test;

pub trait PositionIndexable {
    fn position_index(&self) -> usize;
    const POSITION_SET_SIZE: usize;
}

pub trait OrientationIndexable {
    fn orientation_index(&self) -> usize;
    const ORIENTATION_SET_SIZE: usize;
}

pub trait Indexable: PositionIndexable + OrientationIndexable {
    fn index(&self) -> usize {
        self.position_index() * Self::ORIENTATION_SET_SIZE + self.orientation_index()
    }

    const TOTAL_SET_SIZE: usize = Self::POSITION_SET_SIZE * Self::ORIENTATION_SET_SIZE;
}

impl<T: PositionIndexable + OrientationIndexable> Indexable for T {}

/// Every type that is indexable by position and orientation is automatically also regular [Indexable]
// impl<T: > Indexable for T {
//     fn index(&self) -> usize {
//         self.position_index() * Self::ORIENTATION_SET_SIZE + self.orientation_index()
//     }

//     const TOTAL_SET_SIZE: usize = Self::POSITION_SET_SIZE * Self::ORIENTATION_SET_SIZE;
// }

pub fn disposition_multipliers<
    T: PositionIndexable,
    const N: usize,
    const T_POSITION_SET_SIZE: usize,
>() -> [usize; N] {
    let mut output = [0; N];
    let mut iteration_index = T_POSITION_SET_SIZE - N;
    let mut multiplier = 1;

    for elem in output.iter_mut().rev() {
        *elem = multiplier;
        iteration_index += 1;
        multiplier *= iteration_index;
    }

    output
}

// PERFORMANCE: Maybe this can be done more efficiently
pub fn disposition_choices<
    T: PositionIndexable,
    const N: usize,
    const T_POSITION_SET_SIZE: usize,
>(
    input: &[T; N],
) -> [usize; N] {
    let mut used = [false; T_POSITION_SET_SIZE];
    let mut output = [0; N];

    for (elem, output_elem) in input.iter().zip(output.iter_mut()) {
        let elem_index = elem.position_index();
        let used_previously = used[0..elem_index].iter().filter(|x| **x).count();
        *output_elem = elem_index - used_previously;
        used[elem_index] = true;
    }

    output
}

// Disposition is actually a more accurate term than permutation, since you don't
// have to actually select all the elements in edge "permutations"
// TODO: `const T_POSITION_SET_SIZE: usize` should not be necessary. It has to always be `T::POSITION_SET_SIZE`.
pub fn position_disposition_index<
    T: PositionIndexable,
    const N: usize,
    const T_POSITION_SET_SIZE: usize,
>(
    input: &[T; N],
) -> usize {
    let multipliers = disposition_multipliers::<T, N, T_POSITION_SET_SIZE>();
    let choices = disposition_choices::<T, N, T_POSITION_SET_SIZE>(input);

    // multipliers
    //     .iter()
    //     .zip(choices.iter())
    //     .map(|(multiplier, choice)| multiplier * choice)
    //     .sum()

    // PERFORMANCE: Try the more imperative approach
    let mut output = 0;

    for i in 0..N {
        output += choices[i] * multipliers[i];
    }

    output
}

/// TODO: missing_docs
///
/// # Arguments
///
/// * `input` - Just the input of the function
/// * `is_last_determined` - Whether the last element in the input is determined by the rest of the input (yes for corners, no for edges)
// TODO: Maybe don't make this reversed, since it might have performance implications
pub fn orientation_permutation_index<T: OrientationIndexable, const N: usize>(
    input: &[T; N],
) -> usize {
    let mut output = 0;
    let mut multiplier = 1;

    for elem in input.iter().rev() {
        let elem_index = elem.orientation_index();
        output += elem_index * multiplier;
        multiplier *= T::ORIENTATION_SET_SIZE;
    }

    output
}

impl Cube {
    pub fn edge_partition(&self) -> [&HalfEdges; 2] {
        [
            self.edges[0..6].try_into().expect(
                "`self.edges` has a const length of 12, and [0, 6) is in the range [0, 12)",
            ),
            self.edges[6..12].try_into().expect(
                "`self.edges` has a const length of 12, and [7, 12) is in the range [0, 12)",
            ),
        ]
    }

    /// Returns the indices of the cube's corners and edges.
    ///
    /// Indices are unique and contiguous, so they can be used to index into a table of precomputed values.
    pub fn indices(&self) -> CubeIndices {
        let edges = self.edge_partition();
        let edges = [edges[0], &adjust_second_edges_for_indexing(edges[1])];

        CubeIndices {
            corners: self.corners.index(),
            edges: edges.map(|edges| edges.index()),
        }
    }
}

pub struct CubeIndices {
    pub corners: usize,
    pub edges: [usize; 2],
}

/// This function is used to transform the second half of the edges into a set
/// that can be used for indexing a cache generated by the first set of edges.
///
/// The way it works is by, for each piece, doing the move that in a solved cube would
/// take that piece and move it to the index of that piece minus 6. For example, edge 0
/// is at UF and edge 6 is at BR so this function moves the parameter `edges[0]` (which is
/// edge 6) with U2 B.
///
/// # Warning
///
/// This depends on the structure of `Cube::solved()`.
fn adjust_second_edges_for_indexing(edges: &HalfEdges) -> HalfEdges {
    let mut output = edges.clone();

    for edge in output.iter_mut() {
        Rotation::new(Axis::X, Amount::Double).rotate_edge(edge)
    }

    output
}

#[test]
fn second_edges_are_flip() {
    let [edges_1, edges_2] = Cube::solved().edge_partition();
    let adjusted = &adjust_second_edges_for_indexing(&edges_2);

    assert_eq!(adjusted, edges_1);
}
