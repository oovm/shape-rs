use crate::{Line, Point, Rectangle};
use num_traits::Zero;

mod convert;
mod projection;
#[cfg(feature = "rand")]
mod random;
#[cfg(feature = "wolfram-expr")]
mod wolfram;

/// A trait for checking if a shape is empty
pub trait ValidShape<T>
where
    T: Zero,
{
    fn is_empty(&self) -> bool {
        self.is_empty_under_thousand(T::zero())
    }
    fn is_empty_under_thousand(&self, thousand: T) -> bool;
}

pub trait Shape2D {
    type Value;
    fn is_valid(&self) -> bool;
    fn boundary(&self) -> Rectangle<Self::Value>;
    fn vertices(&self) -> impl Iterator<Item = Point<Self::Value>> + '_;
    fn edges(&self) -> impl Iterator<Item = Line<Self::Value>> + '_;
}

pub trait EqualThousand<T> {
    fn eq_under_thousand(&self, thousand: T) -> bool;
}
