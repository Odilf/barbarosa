#![cfg(test)]

use std::collections::HashSet;

use nalgebra::Vector3;

use crate::cube3::piece::Piece;

use super::*;

#[test]
fn solved_cube_has_pieces_in_all_coordinates() {
    let solved_cube = Cube::new_solved();

    let mut positions: HashSet<_> = solved_cube
        .edges
        .into_iter()
        .map(|piece| piece.coordinates())
        .collect();

    positions.extend(
        solved_cube
            .corners
            .into_iter()
            .map(|piece| piece.coordinates()),
    );

    assert!(
        positions.len() == 20,
        "Expected 20 unique positions, got {}",
        positions.len()
    );

    // Should have a piece at every position with zero or one non-zero coordinates
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
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
    }
}
