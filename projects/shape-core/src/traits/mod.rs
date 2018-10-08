use crate::{Line, Point, Rectangle};

mod convert;
mod projection;
#[cfg(feature = "rand")]
mod random;
#[cfg(feature = "wolfram-expr")]
mod wolfram;

/// The trait for 2D shapes.
///
/// # Arguments
///
/// * `a`:
/// * `b`:
/// * `c`:
///
/// returns: Triangle<T>
///
/// # Examples
///
/// ```
/// # use shape_core::Triangle;
/// ```
pub trait Shape2D {
    /// The value type of the shape.
    type Value;
    /// Returns true if the shape is valid and in normal form.
    fn is_valid(&self) -> bool;
    /// Returns true if the shape successfully normalized.
    fn normalize(&mut self) -> bool {
        self.is_valid()
    }
    /// Returns the boundary of the shape.
    fn boundary(&self) -> Rectangle<Self::Value>;
    /// Returns the owned vertices of the shape.
    ///
    /// Notice that sample only works for non-linear shapes.
    fn vertices(&self, sample: usize) -> impl Iterator<Item = Point<Self::Value>> + '_;
    /// Returns the owned edges of the shape.
    fn edges(&self, sample: usize) -> impl Iterator<Item = Line<Self::Value>> + '_;
}
