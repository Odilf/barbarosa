use nalgebra::Vector3;

use crate::{
    cube3::Cube3,
    generic::{
        piece::{Coordinates, PieceSetDescriptor},
        PieceSet,
    },
    prelude::Piece,
};

pub fn vec_manhattan(a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

pub fn piece_manhattan<P: Piece + Coordinates>(original_pos: P::Position, piece: &P) -> f32 {
    vec_manhattan(&piece.coordinates(), &P::coordinates_pos(original_pos))
}

fn piece_set_manhattan<P: PieceSetDescriptor<N> + Coordinates, const N: usize>(
    set: &PieceSet<P, N>,
) -> f32 {
    set.iter_with_pos()
        .map(|(pos, piece)| piece_manhattan(pos, piece))
        .sum()
}

/// Heuristic based on the manhattan distance
pub fn manhattan(cube: &Cube3) -> f32 {
    let edge_distance = piece_set_manhattan(&cube.edges);
    let corner_distance = piece_set_manhattan(&cube.corners);

    (edge_distance + corner_distance) / (8.0 * 2.0)
}

pub fn manhattan_filtered<P: PieceSetDescriptor<N> + Coordinates, const N: usize>(
    set: &PieceSet<P, N>,
    filter: impl Fn(&P::Position, &P) -> bool,
) -> f32 {
    set.iter_with_pos()
        .filter(|(pos, piece)| filter(pos, piece))
        .map(|(pos, piece)| vec_manhattan(&piece.coordinates(), &P::coordinates_pos(pos)))
        .sum()
}
