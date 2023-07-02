//! 3x3x3 indexing for [mus](super)
//!
//! See [Indexable] for more information

use crate::{
    cube3::{Axis, Cube3},
    cube_n::moves::{
        rotation::{AxisRotation, Rotatable},
        Amount,
    },
};

use super::HalfEdges;

/// Const calculation of the factorial of a number
///
/// If the number is greater than 20, it would overflow so it just returns 0
pub const fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        2..=20 => factorial(n - 1) * n,
        _ => 0,
    }
}

mod implementations;
mod test;

/// Trait for indexing based on position.
///
/// See also [Indexable]
pub trait PositionIndexable {
    /// Returns the index of the position
    fn position_index(&self) -> usize;

    /// The amount of possible positions
    const POSITION_SET_SIZE: usize;
}

/// Trait for indexing based on orientation.
///
/// See also [Indexable]
pub trait OrientationIndexable {
    /// Returns the index of the orientation
    fn orientation_index(&self) -> usize;

    /// The amount of possible orientations
    const ORIENTATION_SET_SIZE: usize;
}

/// Trait for indexing. Every unique instance of a type that implements this trait has to return a
/// unique index, and all indices have to be contiguous (starting from 0). In other words, you can index
/// into an array of size [Self::TOTAL_SET_SIZE] with the index returned by [Self::index()].
///
/// This type requires and is auto-implemented with [PositionIndexable] and [OrientationIndexable]
// TODO: Write about the implementation
pub trait Indexable: PositionIndexable + OrientationIndexable {
    /// Returns the index of the instance. See [Indexable] for more info.
    fn index(&self) -> usize {
        self.position_index() * Self::ORIENTATION_SET_SIZE + self.orientation_index()
    }

    /// The amount of possible instances of this type.
    const TOTAL_SET_SIZE: usize = Self::POSITION_SET_SIZE * Self::ORIENTATION_SET_SIZE;
}

impl<T: PositionIndexable + OrientationIndexable> Indexable for T {}

/// Returns an array of the multipliers used to calculate the index of a disposition
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
fn disposition_choices<T: PositionIndexable, const N: usize, const T_POSITION_SET_SIZE: usize>(
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

/// Returns the index of the disposition (or permutation) of the elements in the input
///
/// Disposition is actually a more accurate term than permutation, since you don't
/// have to actually select all the elements in edge "permutations"
// TODO: Remove `const T_POSITION_SET_SIZE: usize` and use `T::POSITION_SET_SIZE` instead when it's possible.
pub fn position_disposition_index<
    T: PositionIndexable,
    const N: usize,
    const T_POSITION_SET_SIZE: usize,
>(
    input: &[T; N],
) -> usize {
    let multipliers = disposition_multipliers::<T, N, T_POSITION_SET_SIZE>();
    let choices = disposition_choices::<T, N, T_POSITION_SET_SIZE>(input);

    // Imperative approach because it might be faster than zipping?
    let mut output = 0;
    for i in 0..N {
        output += choices[i] * multipliers[i];
    }

    output
}

/// Returns the index of the orientation of the elements in the input
///
/// Assumes that all elements can have any orientation. You should be careful if this is not
/// the case, namely with the 3x3 corners. You should only pass `corners[0..7]` in that case.
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

impl Cube3 {
    fn edge_partition(&self) -> [&HalfEdges; 2] {
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

/// The indices of the cube's corners and edges.
#[allow(missing_docs)]
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
        edge.rotate(&AxisRotation::new(Axis::X, Amount::Double))
    }

    output
}
