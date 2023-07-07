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