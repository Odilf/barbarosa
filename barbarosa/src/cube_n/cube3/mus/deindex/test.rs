#![cfg(test)]

use rand::{rngs::StdRng, Rng, SeedableRng};

use super::*;

use crate::{
    cube3::mus::index::{orientation_permutation_index, position_disposition_index},
    cube_n::AxisMove,
    generic::{alg::Alg, Cube, Movable, Parsable},
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
    let cube = Cube3::solved();
    let indices = cube.indices();
    let deindexed_cube = Cube3::from_indices(indices);

    assert_eq!(cube, &deindexed_cube);
}

#[test]
fn index_deindex_edges() {
    let cube = Cube3::new_solved().moved(&Alg::<AxisMove>::parse("R").unwrap());
    let indices = cube.indices();
    let deindexed_cube = Cube3::from_indices(indices);

    assert_eq!(cube.edges, deindexed_cube.edges);
}

#[test]
fn index_deindex_corners() {
    let cube = Cube3::new_solved().moved(&Alg::<AxisMove>::parse("U").unwrap());
    let indices = cube.indices();
    let deindexed_cube = Cube3::from_indices(indices);

    assert_eq!(cube.corners, deindexed_cube.corners);
}

#[test]
fn random_indexes_deindexes() {
    let original = StdRng::seed_from_u64(69420).gen::<Cube3>();
    let deindexed = Cube3::from_indices(original.indices());

    assert_eq!(original, deindexed);
}

#[test]
fn index_deindex_journey() {
    for _ in 0..10 {
        random_indexes_deindexes();
    }
}
