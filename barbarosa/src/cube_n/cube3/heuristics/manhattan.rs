use nalgebra::Vector3;

use crate::{
    cube3::Cube3,
    cube_n::{Corner, Edge},
    generic::Cube,
};

fn vec_distance(a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn edge_distance(a: &Edge, b: &Edge) -> f32 {
    vec_distance(&a.coordinates(), &b.coordinates())
}

fn corner_distance(a: &Corner, b: &Corner) -> f32 {
    vec_distance(&a.coordinates(), &b.coordinates())
}

fn edge_distances(edges: &[Edge]) -> f32 {
    edges
        .iter()
        .zip(Cube3::solved().edges.iter())
        .map(|(a, b)| edge_distance(a, b))
        .sum()
}

fn corner_distances(corner: &[Corner]) -> f32 {
    corner
        .iter()
        .zip(Cube3::solved().corners.iter())
        .map(|(a, b)| corner_distance(a, b))
        .sum()
}

/// Heuristic based on the manhattan distance
pub fn manhattan(cube: &Cube3) -> f32 {
    let edge_distance = edge_distances(&cube.edges);
    let corner_distance = corner_distances(&cube.corners);

    (edge_distance + corner_distance) / (8.0 * 2.0)
}
