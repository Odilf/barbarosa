//! Random (solvable) cube generation.
//!
//! See [Cube::random()] for more information.

use rand::{seq::SliceRandom, Rng};

use super::Cube;

impl Cube {
    /// Same as [Cube::random()], but with a specified RNG
    pub fn random_with_rng(rng: &mut impl Rng) {
        let mut cube = Cube::new_solved();

        // Move pieces
        cube.edges.shuffle(rng);
        cube.corners.shuffle(rng);

        // Fix piece permutations
        let edge_swap_parity = swap_cycles(&cube.edges, &Self::solved().edges) % 2 == 0;
        let corner_swap_parity = swap_cycles(&cube.corners, &Self::solved().corners) % 2 == 0;

        if edge_swap_parity != corner_swap_parity {
            (cube.edges[0], cube.edges[1]) = (cube.edges[1].clone(), cube.edges[0].clone());
        }

        // Flip pieces
        cube.edges
            .iter_mut()
            .for_each(|edge| edge.oriented = rng.gen());
        cube.corners
            .iter_mut()
            .for_each(|corner| corner.orientation_axis = rng.gen());

        // Fix edge orientation
        let oriented_edges = cube.edges.iter().filter(|edge| edge.oriented).count();
        if oriented_edges % 2 == 1 {
            cube.edges[0].flip();
        }

        // Fix corner orientation
        let oriented_corners: i32 = cube
            .corners
            .iter()
            .map(|corner| corner.orientation_index() as i32)
            .sum();
        let corner_orientation_offset = oriented_corners % 3;
        for _ in 0..corner_orientation_offset {
            cube.corners[0].twist();
        }
    }

    /// Generates a random, solvable cube.
    ///
    /// See also [Cube::random_with_rng] for specifying an RNG
    ///
    /// # Solvability
    ///
    /// The way the implementation works is by generating a random array
    /// of edges and corners with random orientations. This will always represented
    /// a cube, but it might not be solvable.
    ///
    /// There are thre invariants we need to uphold in a 3x3x3 Rubik's cube. We can deduce them by analizing what
    /// a single move does in terms of swaps and changes of orientation.
    ///
    /// A move swaps 4 edges and 4 corners. It's easy to reduce this to 2 edges and 2 corners apparent swaps. However,
    /// this means that the parity of the number of swaps to get all edges in their correct position has to be the one
    /// for corners.
    ///
    /// A move also changes the orientation of pieces. For the edges, a move either flips 0 or 4 edges. This can be
    /// again reduced to 2 apparent flips so there need to be an even number of edges flipped.
    ///
    /// Defining the orientation of a corner is trickier. We can define an "orientation index" as the number of
    /// counter-clockwise rotations needed to get the corner in the correct orientation. Then, we can see that a move
    /// changes the sum of all orientation indices by 0 or 6, which can be reduced to 3. This means that the sum of
    /// all orientation indices needs to be divisible by 3.
    ///
    /// To make sure it's solvable, we do three things:
    ///
    /// 1. We swap `cube.edges[0]` and `cube.edges[1]` if the parity of the edge permutation is different from the parity of the corner permutation.
    /// 2. We flip `cube.edges[0]` if the number of oriented edges is odd.
    /// 3. We twist `cube.corners[0]` such that the sum of corner orientation indices is divisble by 3.
    ///
    ///
    ///
    /// # Note about uniformity of distribution
    ///
    /// It seems to me that this would keep every solvable state at uniform probability. However, maybe the
    /// fact that we're changing the state of the pieces at the start of the already uniform shuffled array
    /// messes with things. I'm pretty sure it doesn't because it's random in the first place, but who knows
    /// lol (i haven't really looked into it yet)
    pub fn random() {
        let mut rng = rand::thread_rng();
        Self::random_with_rng(&mut rng);
    }
}

fn swap_cycles<T: Eq, const N: usize>(given: &[T; N], original: &[T; N]) -> i32 {
    let mut permutations: [Option<usize>; N] = [None; N];
    let mut current_index = 0;
    let mut cycles = 0;

    loop {
        if permutations[current_index].is_none() {
            let Some(first_unvisited) = permutations.iter().position(|x| x.is_none()) else {
				return cycles;
			};

            cycles += 1;
            current_index = first_unvisited;
        }

        let next = original
            .iter()
            .position(|x| x == &given[current_index])
            .unwrap();
        permutations[current_index] = Some(next);
        current_index = next;
    }
}
