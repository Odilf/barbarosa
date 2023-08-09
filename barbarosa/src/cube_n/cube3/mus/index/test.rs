#![cfg(test)]

use itertools::Itertools;

use crate::{
    cube3::{mus::CornersMUS, Corner, Edge},
    generic::Cube,
};

use super::*;

// To make testing position permutations easier
impl PositionIndexable for usize {
    fn position_index(&self) -> usize {
        *self
    }

    const POSITION_SET_SIZE: usize = 6;
}

impl OrientationIndexable for usize {
    fn orientation_index(&self) -> usize {
        *self
    }

    const ORIENTATION_SET_SIZE: usize = 3;
}

#[test]
fn test_disposition_multipliers() {
    let multipliers_corners = disposition_multipliers::<usize, 8, 8>();

    assert_eq!(
        multipliers_corners.to_vec(),
        (0..8).rev().map(factorial).collect_vec()
    );

    let multipliers_edges = disposition_multipliers::<usize, 6, 12>();

    assert_eq!(
        multipliers_edges.to_vec(),
        (6..12)
            .rev()
            .map(|i| factorial(i) / factorial(6))
            .collect_vec()
    );
}

#[test]
fn first_permutation_indices() {
    let permutations = [
        [0, 1, 2, 3, 4, 5],
        [0, 1, 2, 3, 5, 4],
        [0, 1, 2, 4, 3, 5],
        [0, 1, 2, 4, 5, 3],
        [0, 1, 2, 5, 3, 4],
        [0, 1, 2, 5, 4, 3],
    ];

    for (i, perm) in permutations.iter().enumerate() {
        assert_eq!(
            position_disposition_index::<_, 6, { usize::POSITION_SET_SIZE }>(perm),
            i
        );
    }
}

#[test]
fn last_permutation_is_reverse() {
    let permutation = [5, 4, 3, 2, 1, 0];

    assert_eq!(
        position_disposition_index::<_, 6, { usize::POSITION_SET_SIZE }>(&permutation),
        factorial(6) - 1
    );
}

#[test]
fn first_orientation_indices() {
    let orientations = [
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 2],
        [0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 1, 1],
        [0, 0, 0, 0, 0, 1, 2],
        [0, 0, 0, 0, 0, 2, 0],
        [0, 0, 0, 0, 0, 2, 1],
        [0, 0, 0, 0, 0, 2, 2],
    ];

    for (i, orientation) in orientations.iter().enumerate() {
        assert_eq!(orientation_permutation_index(orientation), i);
    }
}

#[test]
fn first_and_last_corner_index() {
    let mut corner = Cube3::SOLVED.corners.pieces()[7].clone();
    corner.twist();
    corner.twist();

    assert_eq!(Cube3::SOLVED.corners.pieces()[0].index(), 0);
    assert_eq!(corner.index(), Corner::TOTAL_SET_SIZE - 1);
}

#[test]
fn first_and_last_edge_index() {
    let first_edge = Cube3::SOLVED.edges.pieces()[0].clone();
    let last_edge = Cube3::SOLVED.edges.pieces()[11].clone().flipped();

    assert_eq!(first_edge.index(), 0);
    assert_eq!(last_edge.index(), Edge::TOTAL_SET_SIZE - 1);
}

#[test]
fn second_edges_adjustment_solved() {
    let [edges_1, edges_2] = Cube3::SOLVED.edge_partition();
    let adjusted = &adjust_second_edges_for_indexing(edges_2);

    assert_eq!(adjusted, edges_1);
}

#[test]
fn first_and_last_corner_set() {
    let first_corners = Cube3::SOLVED.corners.pieces().clone();
    let last_corners = {
        let mut corners = Cube3::SOLVED.corners.pieces().clone();
        corners.reverse();
        corners.iter_mut().for_each(|corner| {
            corner.twist();
            corner.twist();
        });

        corners
    };

    assert_eq!(first_corners.index(), 0);
    assert_eq!(last_corners.index(), CornersMUS::TOTAL_SET_SIZE - 1);
}

#[test]
fn first_and_last_edge_set() {
    let first_edges = Cube3::SOLVED.edge_partition()[0].clone();
    let last_edges = {
        let cube = Cube3::SOLVED;
        let mut edges = cube.edge_partition()[1].clone();
        edges.reverse();
        edges.iter_mut().for_each(|edge| edge.flip());
        edges
    };

    assert_eq!(first_edges.index(), 0);

    assert_eq!(
        last_edges.position_index(),
        <HalfEdgesMUS>::POSITION_SET_SIZE - 1
    );
    assert_eq!(
        last_edges.orientation_index(),
        <HalfEdgesMUS>::ORIENTATION_SET_SIZE - 1
    );
    assert_eq!(last_edges.index(), <HalfEdgesMUS>::TOTAL_SET_SIZE - 1);
}
