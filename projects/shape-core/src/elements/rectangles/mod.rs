use super::*;
mod traits;

mod parallelogram;
pub mod rectangle_2d;
pub mod rectangle_3d;
pub mod square_2d;

/// A parallelogram is a quadrilateral with two pairs of parallel sides.
///
/// # Notice
///
/// The constructor will not check the legality of the parameters, call [`is_valid`] to ensure that the rectangle_2d is legal.
///
/// # Examples
///
/// ```
/// # use shape_core::Square;
/// let rect = Square::new(0.0, 0.0, 1.0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Parallelogram<T> {
    /// The origin points of the parallelogram.
    pub anchor: Point<T>,
    /// The side length of the parallelogram.
    pub side: (Point<T>, Point<T>),
}
