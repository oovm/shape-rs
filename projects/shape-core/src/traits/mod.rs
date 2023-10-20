use crate::{Line, Point, Rectangle};
use std::marker::PhantomData;

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
    /// The value type of the shape.
    type VertexIterator<'a>: Iterator<Item = Point<Self::Value>>
    where
        Self: 'a;
    /// The value type of the shape.
    type LineIterator<'a>: Iterator<Item = Line<Self::Value>>
    where
        Self: 'a;
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
    fn vertices<'a>(&'a self, sample: usize) -> Self::VertexIterator<'a>;
    /// Returns the owned edges of the shape.
    fn edges<'a>(&'a self, sample: usize) -> Self::LineIterator<'a>;
}

/// A placeholder iterator for missing trait bound
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceHolderNodeIterator<T> {
    place_holder: PhantomData<T>,
}

/// A placeholder iterator for missing trait bound
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlaceHolderEdgeIterator<T> {
    place_holder: PhantomData<T>,
}
impl<T> Iterator for PlaceHolderNodeIterator<T> {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<T> Iterator for PlaceHolderEdgeIterator<T> {
    type Item = Line<T>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
