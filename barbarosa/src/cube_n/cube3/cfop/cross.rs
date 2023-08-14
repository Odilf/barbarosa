//! The "C" in CFOP.
//!
//! Functions to solve and manipulate the cross of a 3x3.

use nalgebra::Vector2;

use crate::{
    cube3::Cube3,
    cube_n::{
        pieces::edge::min_moves_to_solve,
        space::{Direction, Face},
        AxisMove, Edge,
    },
    generic::{
        search::{ida::IDASearcher, Searcher},
        Alg, Piece,
    },
};

/// Whether the edge with the given original position is a cross piece of `face`
pub fn is_cross_edge(original_position: &<Edge as Piece>::Position, bottom_face: &Face) -> bool {
    Edge::direction_on_axis(original_position, bottom_face.axis) == Some(bottom_face.direction)
}

/// Returns the number of cross pieces of `face` in `cube`. Between 0 and 4.
pub fn count_cross_pieces(cube: &Cube3, bottom_face: &Face) -> i32 {
    cube.edges
        .iter_with_pos()
        .filter(|(original_pos, edge)| {
            edge.position() == *original_pos
                && is_cross_edge(original_pos, bottom_face)
                && edge.oriented
        })
        .count() as i32
}

/// Gets the original positions of the 4 cross pieces that belong to `bottom_face`
pub fn get_cross_pieces(bottom_face: &Face) -> [<Edge as Piece>::Position; 4] {
    let n1 = bottom_face.axis.next();
    let n2 = n1.next();

    debug_assert_eq!(n1.next().next(), bottom_face.axis);
    debug_assert_eq!(n2.next(), bottom_face.axis);

    [
        (n1, Vector2::new(Direction::Positive, bottom_face.direction)),
        (n1, Vector2::new(Direction::Negative, bottom_face.direction)),
        (n2, Vector2::new(bottom_face.direction, Direction::Positive)),
        (n2, Vector2::new(bottom_face.direction, Direction::Negative)),
    ]
}

/// Solves the cross of `cube` with `bottom_face` as the bottom face.
///
/// It usually produces optimal results, but sometiems it doesn't. To always
/// get an optimal cross, use [`solve_cross_optimally`], which is slower but
/// always optimal.
pub fn solve_cross_fast(cube: &Cube3, bottom_face: &Face) -> Option<(Alg<AxisMove>, Cube3)> {
    solve_cross_directly(cube, bottom_face, |cube| {
        good_cross_heuristic(cube, bottom_face)
    })
}

/// Solves the cross of `cube` with `bottom_face` as the bottom face in the least amount of moves.
///
/// However, it is pretty slow. For a fast alternative that is still almost optimal, use [`solve_cross`]
pub fn solve_cross(cube: &Cube3, bottom_face: &Face) -> Option<(Alg<AxisMove>, Cube3)> {
    solve_cross_directly(cube, bottom_face, |cube| {
        worse_but_admissable_cross_heuristic(cube, bottom_face)
    })
}

fn solve_cross_directly(
    cube: &Cube3,
    bottom_face: &Face,
    heuristic: impl Fn(&Cube3) -> f32,
) -> Option<(Alg<AxisMove>, Cube3)> {
    let searcher = IDASearcher::new(heuristic, Cube3::successors, 200);

    searcher.search(cube, |cube| count_cross_pieces(cube, bottom_face) == 4)
}

/// Returns a heuristic for solving the cross that is pretty good but not admissable. This means
/// that it will sometimes overestimate the number of moves needed to solve the cross which, in
/// turn, means that it will sometimes not find the optimal solution.
pub fn good_cross_heuristic(cube: &Cube3, bottom_face: &Face) -> f32 {
    cube.edges
        .iter_with_pos()
        .filter(|(original, _)| is_cross_edge(&original, bottom_face))
        .map(|(original, edge)| min_moves_to_solve(&original, &edge))
        .sum::<i32>() as f32
        / 2.0 // Constant is kinda arbitrary to improve performance
}

/// Returns a heuristic for solving the cross that is admissable but not very good. This means
/// that it will always find the optimal solution, but it is around an order of magnitude slower
/// than [`good_cross_heuristic`] and way less consistent.
pub fn worse_but_admissable_cross_heuristic(cube: &Cube3, bottom_face: &Face) -> f32 {
    cube.edges
        .iter_with_pos()
        .filter(|(original, _)| is_cross_edge(&original, &bottom_face))
        .map(|(original, edge)| min_moves_to_solve(&original, &edge))
        .max()
        .expect("Edges have 12 items, which is greater 0") as f32
}
