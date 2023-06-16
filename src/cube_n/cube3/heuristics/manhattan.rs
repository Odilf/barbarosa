use nalgebra::Vector3;

use crate::{
    cube3::Cube3,
    generic::{Cube, Piece},
};

fn vec_distance(a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn piece_distance<P: Piece>(a: &P, b: &P) -> f32 {
    vec_distance(&a.coordinates(), &b.coordinates())
}

fn pieces_distance<P: Piece>(a: &[P], b: &[P]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| piece_distance(a, b))
        .sum()
}

/// Heuristic based on the manhattan distance
pub fn manhattan(cube: &Cube3) -> i8 {
    let edge_distance = pieces_distance(&Cube3::solved().edges, &cube.edges);
    let corner_distance = pieces_distance(&Cube3::solved().corners, &cube.corners);

    ((edge_distance + corner_distance) / (8.0 * 2.0)) as i8
}
