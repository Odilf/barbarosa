use rand::prelude::Distribution;

use crate::cube_n::space::Face;
pub use crate::generic::{
    self,
    parse::{self, Parsable},
};

use super::{Amount, AxisMove};

// A wide move of at most depth `N`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WideAxisMove<const N: u32> {
    /// invariant: depth <= N
    depth: u32,
    pub axis_move: AxisMove,
}

impl<const N: u32> generic::Move for WideAxisMove<N> {
    fn inverse(&self) -> Self {
        let mut output = self.clone();
        output.axis_move = output.axis_move.inverse();
        output
    }
}

impl<const N: u32> WideAxisMove<N> {
    pub fn new(face: Face, amount: Amount, depth: u32) -> Result<Self, ()> {
        if depth > N {
            return Err(());
        }

        Ok(Self {
            depth,
            axis_move: AxisMove { face, amount },
        })
    }

    pub fn face(&self) -> &Face {
        &self.axis_move.face
    }

    pub fn amount(&self) -> Amount {
        self.axis_move.amount
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }
}

impl<const N: u32> std::fmt::Display for WideAxisMove<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.depth {
            0 => write!(f, "{}", self.axis_move),
            1 => write!(f, "{}w{}", self.axis_move.face, self.axis_move.amount),
            i => write!(
                f,
                "{}{}w{}",
                i + 1,
                self.axis_move.face,
                self.axis_move.amount
            ),
        }
    }
}

impl<const N: u32> Distribution<WideAxisMove<N>> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> WideAxisMove<N> {
        let face = rng.gen();
        let amount = rng.gen();
        let depth = rng.gen_range(0..=N);
        WideAxisMove::new(face, amount, depth).unwrap()
    }
}
