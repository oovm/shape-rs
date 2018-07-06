use num_traits::Zero;

mod convert;
mod dim2;
mod projection;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;

pub trait Distance<T, RHS> {
    fn distance_to(&self, other: &RHS) -> T;
}

pub trait ValidShape<T>
where
    T: Zero,
{
    fn is_empty(&self) -> bool {
        self.is_empty_under_thousand(T::zero())
    }
    fn is_empty_under_thousand(&self, thousand: T) -> bool;
}

pub trait Area<T> {
    fn area(&self) -> T;
}
