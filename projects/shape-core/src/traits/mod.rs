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

pub trait EqualThousand<T> {
    fn eq_under_thousand(&self, thousand: T) -> bool;
}
