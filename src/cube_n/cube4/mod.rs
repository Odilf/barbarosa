//! 4x4x4 cube

mod test;

use nalgebra::Vector2;

use crate::generic::{self, moves::IntoMove, Cube};

use super::{
    moves::rotation::{AxisRotation, Rotatable},
    pieces::{
        center::{self, corner::CornerCenter},
        corner, wing,
    },
    space::{Axis, Direction, Face},
    AxisMove, Corner, Edge, WideAxisMove, Wing,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cube4 {
    corners: [Corner; 8],
    wings: [Wing; 24],
    centers: [CornerCenter; 24],
}

impl generic::Cube for Cube4 {
    fn solved() -> &'static Self {
        &SOLVED
    }

    fn is_solved(&self) -> bool
    where
        Self: 'static,
    {
        let corners_solved = self.corners == Self::solved().corners;
        let wings_solved = self.wings == Self::solved().wings;
        let centers_solved = self
            .centers
            .iter()
            .zip(Self::solved().centers.iter())
            .all(|(current, original)| current.is_solved(original));

        corners_solved && wings_solved && centers_solved
    }
}

impl IntoMove for Cube4 {
    type Move = WideAxisMove<1>;
}

impl generic::Movable<WideAxisMove<1>> for Cube4 {
    fn apply(&mut self, m: &WideAxisMove<1>) {
        self.corners.apply(&m.axis_move);

        self.centers
            .iter_mut()
            .filter(|cc| cc.in_wide_move(1, m))
            .for_each(|cc| cc.rotate(&AxisRotation::from(&m.axis_move)));

        self.wings
            .iter_mut()
            .filter(|wing| wing.in_wide_move(1, m))
            .for_each(|wing| wing.rotate(&AxisRotation::from(&m.axis_move)));
    }
}

impl Cube4 {
    /// Assert that the 4x4 cube is in a consistent state. Right now it only checks that it contains all wings
    pub fn assert_consistent(&self) {
        for solved_wing in &Cube4::solved().wings {
            assert!(
                self.wings.contains(solved_wing),
                "{:#?} is missing",
                solved_wing
            )
        }
    }

    pub fn wing_at(
        &self,
        normal_axis: Axis,
        slice_position: Vector2<Direction>,
        normal_direction: Direction,
    ) -> &Wing {
        let target = Wing::new(normal_axis, slice_position, normal_direction);

        self.wings
            .iter()
            .zip(Self::solved().wings.iter())
            .find_map(|(current, original)| {
                if current == &target {
                    Some(original)
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn wing_at_faces(&self, faces: [Face; 2], normal_direction: Direction) -> Option<&Wing> {
        let (normal_axis, slice_position) = Edge::position_from_faces(faces).ok()?;

        Some(self.wing_at(normal_axis, slice_position, normal_direction))
    }

    pub fn position_of_wing(&self, target: &Wing) -> &Wing {
        self.wings
            .iter()
            .zip(Self::solved().wings.iter())
            .find_map(|(current, original)| {
                if original == target {
                    Some(current)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

impl generic::Movable<AxisMove> for Cube4 {
    fn apply(&mut self, m: &AxisMove) {
        let mov = m.clone().widen(0).unwrap();
        self.apply(&mov)
    }
}

const SOLVED: Cube4 = Cube4 {
    corners: corner::SOLVED,
    wings: wing::SOLVED,
    centers: center::corner::SOLVED,
};
