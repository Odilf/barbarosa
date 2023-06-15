//! Collection of functions to assert if cube invariants are upheld and to fix them.

use super::Corner;

pub fn fix_corner_multiplicity(corners: &mut [Corner; 8]) {
    let oriented_corners: i32 = corners
        .iter()
        .map(|corner| corner.orientation_index() as i32)
        .sum();
    let corner_orientation_offset = oriented_corners % 3;
    for _ in 0..corner_orientation_offset {
        corners[7].twist();
    }
}

// TODO: Move all invariants from `random` to here
