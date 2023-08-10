//! Generic utilities

use nalgebra::{Vector2, Vector3};

use crate::cube_n::space::Axis;

/// Returns the original item at the current position of `target`
///
/// # Examples
///
/// ```rust
/// use barbarosa::generic::utils::item_at;
///
/// let current = vec!['a', 'd', 'b', 'c'];
/// let original = vec!['a', 'b', 'c', 'd'];
/// let target = 'c';
///
/// assert_eq!(item_at(&target, &current, &original), Some(&'d'));
/// ```
///
/// See also [position_of_item]
pub fn item_at<'b, T: PartialEq>(target: &T, current: &[T], original: &'b [T]) -> Option<&'b T> {
    current
        .iter()
        .zip(original.iter())
        .find_map(|(current, original)| {
            if target == current {
                Some(original)
            } else {
                None
            }
        })
}

/// Returns the current position of `target` in `original`
///
/// # Examples
///
/// ```rust
/// use barbarosa::generic::utils::position_of_item;
///
/// let current = vec!['a', 'd', 'b', 'c'];
/// let original = vec!['a', 'b', 'c', 'd'];
/// let target = 'c';
///
/// assert_eq!(position_of_item(&target, &current, &original), Some(&'b'));
/// ```
///
/// See also [item_at]
pub fn position_of_item<'a, T: PartialEq>(
    target: &T,
    current: &'a [T],
    original: &[T],
) -> Option<&'a T> {
    current
        .iter()
        .zip(original.iter())
        .find_map(|(current, original)| {
            if original == target {
                Some(current)
            } else {
                None
            }
        })
}

macro_rules! map_array_const {
    ($array:expr, $array_length:literal, $f:expr) => {
        {
            const_assert_eq!($array.len(), $array_length);
            let mut output = arr![$f ($array[0]); $array_length];
            let mut i = 0;

            while i < $array.len() {
                output[i] = $f ($array[i]);

                i += 1;
            };

            output
        }
    };
}

pub(crate) use map_array_const;

/// Returns the coordinates of the plane determined by a normal
pub fn slice_coords<T: Copy>(vector: Vector3<T>, normal: Axis) -> Vector2<T> {
    let (x, y) = match normal {
        Axis::X => (1, 2),
        Axis::Y => (0, 2),
        Axis::Z => (0, 1),
    };

    Vector2::new(vector[x], vector[y])
}
