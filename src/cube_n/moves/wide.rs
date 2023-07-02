use crate::cube_n::space::Face;
pub use crate::generic::{
    self,
    parse::{self, Parsable},
};

use super::{Amount, AxisMove};

// A wide move of at most depth `N`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WideAxisMove<const N: u32> {
    /// invariant: depth <= Nxx
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
    pub fn new(face: Face, amount: Amount, depth: u32) -> Option<Self> {
        if depth > N {
            return None;
        }

        Some(Self {
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
