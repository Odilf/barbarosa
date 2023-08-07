#![cfg(test)]

use std::collections::HashSet;

use itertools::iproduct;
use nalgebra::{vector, Vector3};

use crate::{
    cube_n::space::Direction,
    generic::{Cube, Movable, Parsable, Piece},
};

use super::*;

#[test]
fn solved_cube_has_pieces_in_all_coordinates() {
    let solved_cube = Cube3::new_solved();

    let mut positions: HashSet<_> = solved_cube
        .edges
        .pieces()
        .into_iter()
        .map(|piece| Edge::coordinates(&piece.position()).map(|i| i as i32))
        .collect();

    positions.extend(
        solved_cube
            .corners
            .pieces()
            .into_iter()
            .map(|piece| Corner::coordinates(&piece.position()).map(|i| i as i32)),
    );

    assert!(
        positions.len() == 20,
        "Expected 20 unique positions, got {}",
        positions.len()
    );

    // Should have a piece at every position with zero or one non-zero coordinates
    for (x, y, z) in iproduct!(-1..=1, -1..=1, -1..=1) {
        if [x, y, z].iter().filter(|&coord| *coord == 0).count() < 2 {
            assert!(
                positions.contains(&Vector3::new(x, y, z)),
                "Expected position ({}, {}, {}) to be visited",
                x,
                y,
                z
            );
        }
    }
}

#[test]
fn random_cube() {
    let _random: Cube3 = rand::random();
}

#[test]
fn test_f() {
    let mut cube = Cube3::new_solved();

    let target = Edge::oriented(Axis::Y, vector![Direction::Positive, Direction::Positive]);
    let target_index = cube
        .edges
        .iter()
        .position(|current| current == &target)
        .unwrap();

    cube.apply(&AxisMove::parse("F").unwrap());

    let rotated = &cube.edges.pieces()[target_index];

    assert_eq!(rotated.normal_axis, Axis::X);
    assert_eq!(
        rotated.slice_position,
        vector![Direction::Negative, Direction::Positive]
    );
}
