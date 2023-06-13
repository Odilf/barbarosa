//! Collection of heuristics for the A* search algorithm.

pub use manhattan::manhattan;

mod manhattan {
    use nalgebra::Vector3;

    use crate::cube3::{space::Direction, Corner, Cube, Edge, Piece};

    fn abs_dif(a: Direction, b: Direction) -> i8 {
        if a == b {
            0
        } else {
            2
        }
    }

    fn dir_vec_distance(a: &Vector3<Direction>, b: &Vector3<Direction>) -> i8 {
        a.iter().zip(b.iter()).map(|(a, b)| abs_dif(*a, *b)).sum()
    }

    fn corner_distance(a: &Corner, b: &Corner) -> i8 {
        dir_vec_distance(&a.position, &b.position)
    }

    fn edge_distance(a: &Edge, b: &Edge) -> i8 {
        a.position()
            .iter()
            .zip(b.position().iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    /// Heuristic based on the manhattan distance
    pub fn manhattan(cube: &Cube) -> i8 {
        let edge_distance: i8 = Cube::solved()
            .edges
            .iter()
            .zip(cube.edges.iter())
            .map(|(a, b)| edge_distance(a, b))
            .sum();

        let corner_distance: i8 = Cube::solved()
            .corners
            .iter()
            .zip(cube.corners.iter())
            .map(|(a, b)| corner_distance(a, b))
            .sum();

        (edge_distance + corner_distance) / (8 * 2)
    }
}
