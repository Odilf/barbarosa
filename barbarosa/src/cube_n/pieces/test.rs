#![cfg(test)]

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    generic::search::{ida::IDASearcher, Searcher},
    prelude::{Cube3, Piece},
};

use super::{edge::EdgeSet, *};

#[test]
fn edge_min_to_solve() {
    let mut rng = StdRng::seed_from_u64(69420);

    for _ in 0..10 {
        let cube: Cube3 = rng.gen();
        let target = EdgeSet::REFERENCE_POSITIONS[5];

        println!("Cube is \n{cube}");

        let min = edge::min_moves_to_solve(&target, cube.edges.piece_originally_at(&target));
        let searcher = IDASearcher::new(|_| 0.0, Cube3::successors, 20);

        let solution = searcher
            .search(&cube, |cube| {
                cube.edges
                    .iter_with_pos()
                    .all(|(original, edge)| original != target || edge.is_solved(&original))
            })
            .unwrap();

        println!("Found solution {solution}");

        assert_eq!(solution.moves.len(), min as usize);
    }
}
