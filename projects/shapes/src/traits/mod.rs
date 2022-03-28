mod convert;
mod projection;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;

pub trait Distance {
    type Other;
    fn distance(&self, other: &Self::Other) -> f32;
}
