//! Corner piece of the cube.

use arr_macro::arr;
use cartesian_array_product::cartesian_array_map;
use nalgebra::Vector3;
use static_assertions::const_assert_eq;

use crate::{
    cube_n::{
        moves::rotation::{AxisRotation, Rotatable},
        space::{Axis, Direction, Face},
        AxisMove, Vec3,
    },
    generic::{
        self, moves::impl_movable_array, piece::PieceSetDescriptor, utils::map_array_const,
        PieceSet,
    },
};

/// A corner piece of the cube.
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Corner {
    /// The position of the corner piece, relative to the center of the cube.
    pub position: Vec3,

    /// The orientation of the corner piece, determined by the axis of the
    /// sticker that is originally on the Y (U-D) axis (usually white-yellow)
    pub orientation_axis: Axis,
}

impl generic::Piece for Corner {
    type Position = Vec3;

    fn position(&self) -> Self::Position {
        self.position
    }

    fn is_solved(&self, original_pos: &Self::Position) -> bool {
        self.position() == *original_pos && self.is_oriented()
    }
}

impl PieceSetDescriptor<8> for Corner {
    const REFERENCE_POSITIONS: [Self::Position; 8] = {
        use Direction::*;

        cartesian_array_map!(
            [Positive, Negative], [Positive, Negative], [Positive, Negative];
            Vector3::new
        )
    };

    const SOLVED: [Self; 8] = map_array_const!(Corner::REFERENCE_POSITIONS, 8, Corner::oriented);
}

impl Rotatable for Corner {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.position.rotate(rotation);
        self.orientation_axis.rotate(rotation);
    }
}

impl generic::Movable<AxisMove> for Corner {
    fn apply(&mut self, m: &AxisMove) {
        if m.face.contains_vector(&self.position) {
            let rotation = AxisRotation::from(m);
            self.rotate(&rotation);
        }
    }
}

impl_movable_array!(Corner, AxisMove);

impl generic::piece::Coordinates for Corner {
    fn coordinates_pos(position: Self::Position) -> Vector3<f32> {
        position.map(|dir| dir.scalar() as f32)
    }
}

impl Corner {
    /// Creates a new [`Corner`] with the given position and orientation axis.
    pub const fn new(position: Vec3, orientation_axis: Axis) -> Self {
        Self {
            position,
            orientation_axis,
        }
    }

    /// The axis that is considered to be the correct orientation axis for corners
    pub const ORIENTED_AXIS: Axis = Axis::Y;

    /// Creates a new [Corner] with the given position and correct orientation.
    pub const fn oriented(position: Vec3) -> Self {
        Self {
            position,
            orientation_axis: Self::ORIENTED_AXIS,
        }
    }

    /// Returns the faces that the corner is on
    pub fn faces(&self) -> [Face; 3] {
        [
            Face::new(Axis::X, self.position.x),
            Face::new(Axis::Y, self.position.y),
            Face::new(Axis::Z, self.position.z),
        ]
    }

    fn is_even_position_parity(position: &Vec3) -> bool {
        position
            .iter()
            .filter(|dir| dir == &&Direction::Negative)
            .count()
            % 2
            == 0
    }

    /// Amount of counter-clockwise rotations needed for an oriented corner to get
    /// to the current orientation. i think
    // TODO: Check the "i think" part
    pub fn orientation_index(&self) -> usize {
        let even_parity = Self::is_even_position_parity(&self.position);
        let axis_index = self.orientation_axis as i32;

        if even_parity {
            (axis_index - 1).rem_euclid(3) as usize
        } else {
            (1 - axis_index).rem_euclid(3) as usize
        }
    }

    /// Gets the axis that is a counterclockwise rotation from the current axis
    /// around the given position. So the thing used for twisting corners
    pub fn next_axis(position: &Vec3, axis: &Axis) -> Axis {
        if Self::is_even_position_parity(position) {
            axis.next()
        } else {
            axis.prev()
        }
    }

    /// Twists a corner counter-clockwise
    pub fn twist(&mut self) {
        self.orientation_axis = if Self::is_even_position_parity(&self.position) {
            self.orientation_axis.next()
        } else {
            self.orientation_axis.prev()
        };
    }

    /// Whether the corner is oriented. This is true if the orientation axis is
    /// the X axis.
    ///
    /// This method might be ever so slightly faster then doing `corner.orientation_index() == 0`.
    pub fn is_oriented(&self) -> bool {
        self.orientation_axis == Self::ORIENTED_AXIS
    }
}

impl std::fmt::Debug for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [f1, f2, f3] = self.faces();

        write!(
            f,
            "Corner {{ faces: {f1}{f2}{f3}, orientation_axis: {:?})",
            self.orientation_axis
        )
    }
}

/// The piece set of 8 [`Corner`]s.
pub type CornerSet = PieceSet<Corner, 8>;
