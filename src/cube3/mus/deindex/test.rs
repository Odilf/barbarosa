#![cfg(test)]

use rand::{rngs::StdRng, SeedableRng};

use super::*;

use crate::cube3::{
    moves::alg::parse_alg,
    mus::index::{
        orientation_permutation_index, position_disposition_index,
    },
    Cube,
};

#[test]
fn index_deindex_orientations() {
    let orientations = [2, 2, 0, 2, 2, 2];
    let index = orientation_permutation_index(&orientations);
    let deindexed = deindex_orientations::<usize, 6>(index);

    assert_eq!(orientations, deindexed);
}

#[test]
fn index_deindex_positions() {
    let orientations = [4, 3, 5, 11, 10, 9];
    let index = position_disposition_index::<_, 6, 12>(&orientations);
    let deindexed = deindex_positions::<usize, 6, 12>(index);

    assert_eq!(orientations, deindexed);
}

#[test]
fn index_deindex_solved() {
    let cube = Cube::solved();
    let indices = cube.indices();
    let deindexed_cube = Cube::from_indices(indices);

    assert_eq!(cube, &deindexed_cube);
}

#[test]
fn index_deindex_edges() {
    let cube = Cube::from(&parse_alg("R").unwrap());
    let indices = cube.indices();
    let deindexed_cube = Cube::from_indices(indices);

    assert_eq!(cube.edges, deindexed_cube.edges);
}

#[test]
fn index_deindex_corners() {
    let cube = Cube::from(&parse_alg("U").unwrap());
    let indices = cube.indices();
    let deindexed_cube = Cube::from_indices(indices);

    assert_eq!(cube.corners, deindexed_cube.corners);
}

#[test]
fn random_indexes_deindexes() {
    let cube = Cube::random_with_rng(&mut StdRng::seed_from_u64(69420));
    let indices = cube.indices();
    let deindexed_cube = Cube::from_indices(indices);

    assert_eq!(cube.corners, deindexed_cube.corners);
}

#[test]
// #[ignore = "Not implemented (?)"]
fn index_deindex_journey() {
    for _ in 0..10 {
        random_indexes_deindexes();
    }
}
