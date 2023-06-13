use crate::cube3::{Cube, Edge};

pub const fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        2..=20 => factorial(n - 1) * n,
        _ => 0,
    }
}

mod test;
mod implementations;

pub trait PositionIndexable {
    fn position_index(&self) -> usize;
    const POSITION_SET_SIZE: usize;
}

pub trait OrientationIndexable {
    fn orientation_index(&self) -> usize;
    const ORIENTATION_SET_SIZE: usize;
}

pub trait Indexable {
    fn index(&self) -> usize;

    const TOTAL_SET_SIZE: usize;
}

/// Every type that is indexable by position and orientation is automatically also regular [Indexable]
impl<T: PositionIndexable + OrientationIndexable> Indexable for T {
    fn index(&self) -> usize {
        self.position_index() * Self::ORIENTATION_SET_SIZE + self.orientation_index()
    }

    const TOTAL_SET_SIZE: usize = Self::POSITION_SET_SIZE * Self::ORIENTATION_SET_SIZE;
}

fn disposition_multipliers<T: PositionIndexable, const N: usize, const T_POSITION_SET_SIZE: usize>() -> [usize; N] {
    let mut output = [0; N];
    let mut iteration_index = T_POSITION_SET_SIZE - N;
    let mut multiplier = 1;

    for elem in output.iter_mut().rev() {
        *elem = multiplier;
        iteration_index += 1;
        multiplier *= iteration_index;
    }

    output
}

// PERFORMANCE: Maybe this can be done more efficiently
fn disposition_choices<T: PositionIndexable, const N: usize, const T_POSITION_SET_SIZE: usize>(input: &[T; N]) -> [usize; N] {
    let mut used = [false; T_POSITION_SET_SIZE];
    let mut output = [0; N];

    for (elem, output_elem) in input.iter().zip(output.iter_mut()) {
        let elem_index = elem.position_index();
        let used_previously = used[0..elem_index].iter().filter(|x| **x).count();
        *output_elem = elem_index - used_previously;
        used[elem_index] = true;
    }

    output
}

// Disposition is actually a more accurate term than permutation, since you don't
// have to actually select all the elements in edge "permutations"
// TODO: `const T_POSITION_SET_SIZE: usize` should not be necessary. It has to always be `T::POSITION_SET_SIZE`.
fn position_disposition_index<T: PositionIndexable, const N: usize, const T_POSITION_SET_SIZE: usize>(input: &[T; N]) -> usize {
    let multipliers = disposition_multipliers::<T, N, T_POSITION_SET_SIZE>();
    let choices = disposition_choices::<T, N, T_POSITION_SET_SIZE>(input);

    multipliers.iter().zip(choices.iter()).map(|(multiplier, choice)| multiplier * choice).sum()

    // PERFORMANCE: Try the more imperative approach
    // let mut output = 0;

    // for i in 0..N {
    //     output += choices[i] * multipliers[i];
    // }

    // output
}

/// TODO: missing_docs
///
/// # Arguments
///
/// * `input` - Just the input of the function
/// * `is_last_determined` - Whether the last element in the input is determined by the rest of the input (yes for corners, no for edges)
// TODO: Maybe don't make this reversed, since it might have performance implications
fn orientation_permutation_index<T: OrientationIndexable, const N: usize>(
    input: &[T; N],
    is_last_determined: bool,
) -> usize {
    let mut output = 0;
    let mut multiplier = 1;

    let iter = if is_last_determined {
        input[0..N - 1].iter().rev()
    } else {
        input.iter().rev()
    };

    for elem in iter {
        let elem_index = elem.orientation_index();
        output += elem_index * multiplier;
        multiplier *= T::ORIENTATION_SET_SIZE;
    }

    output
}

impl Cube {
    fn edge_partition(&self) -> [&[Edge; 6]; 2] {
        [
            self.edges[0..6].try_into().expect("`self.edges` has a const length of 12, and [0, 6) is in the range [0, 12)"),
            self.edges[6..12].try_into().expect("`self.edges` has a const length of 12, and [7, 12) is in the range [0, 12)"),
        ]
    }

    /// Returns the indices of the cube's corners and edges.
    /// 
    /// Indices are unique and contiguous, so they can be used to index into a table of precomputed values.
    pub fn indices(&self) -> CubeIndices {
        CubeIndices {
            corners: self.corners.index(),
            edges: self.edge_partition().map(|x| x.index()),
        }
    }
}

pub struct CubeIndices {
    pub corners: usize,
    pub edges: [usize; 2]
}
