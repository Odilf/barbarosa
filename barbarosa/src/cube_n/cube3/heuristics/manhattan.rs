use nalgebra::Vector3;

use crate::{
    cube3::Cube3,
    cube_n::{Corner, Edge},
    generic::{Piece, PieceSet},
};

fn vec_manhattan(a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn piece_set_manhattan<P: Piece<N>, const N: usize>(
    set: &PieceSet<P, N>,
    coords: impl Fn(&P::Position) -> Vector3<f32>,
) -> f32 {
    set.iter()
        .map(|(pos, piece)| vec_manhattan(&coords(&piece.position()), &coords(&pos)))
        .sum()
}

/// Heuristic based on the manhattan distance
pub fn manhattan(cube: &Cube3) -> f32 {
    let edge_distance = piece_set_manhattan(&cube.edges, Edge::coordinates);
    let corner_distance = piece_set_manhattan(&cube.corners, Corner::coordinates);

    (edge_distance + corner_distance) / (8.0 * 2.0)
}
