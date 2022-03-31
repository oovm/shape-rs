mod convert;
mod projection;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;

pub trait Distance<T, RHS> {
    fn distance_to(&self, other: &RHS) -> T;
}
