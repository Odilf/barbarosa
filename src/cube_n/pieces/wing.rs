//! The wing pieces for 4x4+. See [Wing] for more info.

mod test;

use nalgebra::{vector, Vector2};

use crate::{
    cube_n::{
        moves::{
            rotation::{AxisRotation, Rotatable},
            Amount,
        },
        space::{Axis, Direction, Face},
        WideAxisMove,
    },
    generic,
};

use super::{edge::EdgeFromFacesError, Edge};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Wing {
    // TODO: This is very bodgy lol
    // TODO: Also make this private
    pub corresponding_edge: Edge,
}

impl generic::Piece for Wing {}

impl Rotatable for Wing {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.corresponding_edge.rotate(rotation);

        if rotation.axis == self.normal_axis() && rotation.amount != Amount::Double {
            self.corresponding_edge.oriented = !self.corresponding_edge.oriented;
        }

        // println!("Rotated to {:#?}\n", self.corresponding_edge);
    }
}

impl Wing {
    pub fn normal_axis(&self) -> Axis {
        self.corresponding_edge.normal_axis
    }

    pub fn slice_position(&self) -> Vector2<Direction> {
        self.corresponding_edge.slice_position
    }

    pub fn direction_along_normal(&self) -> Direction {
        let is_x_axis = self.corresponding_edge.normal_axis == Axis::X;
        let is_even_position_parity =
            self.corresponding_edge.slice_position.x == self.corresponding_edge.slice_position.y;

        if is_x_axis ^ is_even_position_parity ^ self.corresponding_edge.oriented {
            Direction::Negative
        } else {
            Direction::Positive
        }
    }
}

pub fn in_wide_move<const N: u32>(wing: &Wing, wing_depth: u32, m: &WideAxisMove<N>) -> bool {
    let wing_edge = &wing.corresponding_edge;
    // If just on the same face
    if m.axis_move.face.contains_edge(&wing.corresponding_edge) {
        return true;
    }

    // If on parallel slices (so, same normal)
    if wing_edge.normal_axis == m.face().axis {
        // If it's on the right depth
        if wing_depth <= m.depth() && m.face().direction == wing.direction_along_normal() {
            return true;
        }
    }

    false
}

impl Wing {
    pub fn new(
        normal_axis: Axis,
        slice_position: Vector2<Direction>,
        normal_direction: Direction,
    ) -> Self {
        let corresponding_edge = Edge {
            normal_axis,
            slice_position,
            oriented: true,
        };

        let mut output = Self { corresponding_edge };

        // Flip edge if it's not in the right direction.
        // TODO: Should refactor this logic to make use of it "directly"
        if output.direction_along_normal() != normal_direction {
            output.corresponding_edge = output.corresponding_edge.flipped();
        }

        output
    }

    pub fn from_faces(
        faces: [Face; 2],
        normal_direction: Direction,
    ) -> Result<Self, EdgeFromFacesError> {
        let (normal_axis, slice_position) = Edge::position_from_faces(faces)?;
        Ok(Wing::new(normal_axis, slice_position, normal_direction))
    }
}

impl std::fmt::Debug for Wing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [f1, f2] = self.corresponding_edge.faces();
        // let face_string = format!("{}{}", faces[0], faces[1]);

        // f.debug_struct("Wing")
        //     .field("faces", &face_string)
        //     .field("direction_along_normal", &self.direction_along_normal())
        //     .finish()

        write!(
            f,
            "Wing {{ faces: {f1}{f2}, direction_along_normal: {:?} }}",
            self.direction_along_normal()
        )
    }
}

pub const SOLVED: [Wing; 24] = {
    use Axis::*;
    use Direction::*;

    [
        Wing {
            corresponding_edge: Edge::oriented(X, vector![Positive, Positive]),
        },
        Wing {
            corresponding_edge: Edge::oriented(X, vector![Positive, Negative]),
        },
        Wing {
            corresponding_edge: Edge::oriented(Y, vector![Positive, Positive]),
        },
        Wing {
            corresponding_edge: Edge::oriented(Y, vector![Positive, Negative]),
        },
        Wing {
            corresponding_edge: Edge::oriented(Z, vector![Positive, Positive]),
        },
        Wing {
            corresponding_edge: Edge::oriented(Z, vector![Negative, Positive]),
        },
        Wing {
            corresponding_edge: Edge::oriented(X, vector![Negative, Negative]),
        },
        Wing {
            corresponding_edge: Edge::oriented(X, vector![Negative, Positive]),
        },
        Wing {
            corresponding_edge: Edge::oriented(Y, vector![Negative, Positive]),
        },
        Wing {
            corresponding_edge: Edge::oriented(Y, vector![Negative, Negative]),
        },
        Wing {
            corresponding_edge: Edge::oriented(Z, vector![Positive, Negative]),
        },
        Wing {
            corresponding_edge: Edge::oriented(Z, vector![Negative, Negative]),
        },
        Wing {
            corresponding_edge: Edge::oriented(X, vector![Positive, Positive]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(X, vector![Positive, Negative]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(Y, vector![Positive, Positive]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(Y, vector![Positive, Negative]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(Z, vector![Positive, Positive]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(Z, vector![Negative, Positive]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(X, vector![Negative, Negative]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(X, vector![Negative, Positive]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(Y, vector![Negative, Positive]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(Y, vector![Negative, Negative]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(Z, vector![Positive, Negative]).flipped(),
        },
        Wing {
            corresponding_edge: Edge::oriented(Z, vector![Negative, Negative]).flipped(),
        },
    ]
};
