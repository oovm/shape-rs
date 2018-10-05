use crate::{Line, Point, Rectangle};
use num_traits::Zero;

mod convert;
mod dim2;
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
    fn vertices<'i, I>(&'i self) -> I
    where
        I: Iterator<Item = Point<&'i Self::Value>>;
    fn edges<'i, I>(&'i self) -> I
    where
        I: Iterator<Item = Line<&'i Self::Value>>;
}

pub trait EqualThousand<T> {
    fn eq_under_thousand(&self, thousand: T) -> bool;
}
