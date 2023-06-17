//! Collection of functions to assert if cube invariants are upheld and to fix them.
//!
//! TODO: This docs are written for a 3x3x3 cube, but they should be valid for any NxNxN cube.
//!
//! Mainly used for [Cube::random()](crate::generic::Cube::random())
//!
//! There are thre invariants we need to uphold in a 3x3x3 Rubik's cube. We can deduce them by analizing what
//! a single move does in terms of swaps and changes of orientation.
//!
//! A move swaps 4 edges and 4 corners. It's easy to reduce this to 2 edges and 2 corners apparent swaps. However,
//! this means that the parity of the number of swaps to get all edges in their correct position has to be the one
//! for corners.
//!
//! A move also changes the orientation of pieces. For the edges, a move either flips 0 or 4 edges. This can be
//! again reduced to 2 apparent flips so there need to be an even number of edges flipped.
//!
//! Defining the orientation of a corner is trickier. We can define an "orientation index" as the number of
//! counter-clockwise rotations needed to get the corner in the correct orientation. Then, we can see that a move
//! changes the sum of all orientation indices by 0 or 6, which can be reduced to 3. This means that the sum of
//! all orientation indices needs to be divisible by 3.
//!
//! To make sure it's solvable, provide three methods:
//!
//! - Swapping `cube.edges[0]` and `cube.edges[1]` if the parity of the edge permutation is different from the parity of the corner permutation.
//! - Flipping `cube.edges[11]` if the number of oriented edges is odd.
//! - Twisting `cube.corners[7]` such that the sum of corner orientation indices is divisble by 3.
//!
//! PS: The reason to change the orientation of the last piece is because it makes implementing
//! [mus::index::OrientationIndexable](super::cube3::mus::index::OrientationIndexable) nicer for corners.

use std::fmt::Debug;

use crate::cube_n::{cube3::mus::index::PositionIndexable, Corner, Cube3, Edge};

/// Swaps `cube.edges[0]` and `cube.edges[1]` if the parity of the edge permutation is different from the parity of the corner permutation.
///
/// See the [module-level documentation](self) for more info.
pub fn fix_swap_parity(cube: &mut Cube3) {
    let edge_swap_parity = swap_cycles(&cube.edges) % 2 == 0;
    let corner_swap_parity = swap_cycles(&cube.corners) % 2 == 0;

    if edge_swap_parity != corner_swap_parity {
        (cube.edges[0], cube.edges[1]) = (cube.edges[1].clone(), cube.edges[0].clone());
    }
}

fn swap_cycles<T: PositionIndexable + PartialEq + Debug, const N: usize>(values: &[T; N]) -> i32 {
    let mut permutations: [Option<usize>; N] = [None; N];
    let mut current_index = 0;
    let mut cycles = 0;

    loop {
        if permutations[current_index].is_some() {
            let Some(first_unvisited) = permutations.iter().position(|x| x.is_none()) else {
				return cycles;
			};

            cycles += 1;
            current_index = first_unvisited;
        }

        let next = values[current_index].position_index();

        permutations[current_index] = Some(next);
        current_index = next;
    }
}

/// Flips `cube.edges[11]` if the number of oriented edges is odd.
///
/// See the [module-level documentation](self) for more info.
pub fn fix_edge_flip_parity(edges: &mut [Edge; 12]) {
    let oriented_edges = edges.iter().filter(|edge| edge.oriented).count();
    if oriented_edges % 2 == 1 {
        edges[11].flip();
    }
}

/// Twists `cube.corners[7]` such that the sum of corner orientation indices is divisble by 3.
///
/// See the [module-level documentation](self) for more info.
pub fn fix_corner_multiplicity(corners: &mut [Corner; 8]) {
    let oriented_corners: i32 = corners
        .iter()
        .map(|corner| corner.orientation_index() as i32)
        .sum();

    let corner_orientation_offset = (-oriented_corners).rem_euclid(3);

    for _ in 0..corner_orientation_offset {
        corners[7].twist();
    }

    // Assert that orientation is actually fixed. Useful to have because corner orientation can be surprisingly tricky
    debug_assert!({
        let oriented_corners: i32 = corners
            .iter()
            .map(|corner| corner.orientation_index() as i32)
            .sum();

        oriented_corners % 3 == 0
    });
}
