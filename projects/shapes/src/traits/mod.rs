use num::{traits::real::Real, Num, One, Zero};

mod convert;
mod projection;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;

pub trait Distance {
    type Other;
    fn distance(&self, other: &Self::Other) -> f32;
}

pub trait Transform2D<T>
where
    Self: Sized,
    T: Zero + One + Real,
{
    fn transform(self, matrix: &[T; 9]) -> Self;
    #[rustfmt::skip]
    fn rotate(self, angle: T) -> Self {
        self.transform(&[
            //
             angle.cos(), angle.sin(), T::zero(),
            -angle.sin(), angle.cos(), T::zero(),
            T::zero(),    T::zero(),   T::one(),
        ])
    }
    fn translate(self, x: f32, y: f32) -> Self {
        self.transform(&[T::one(), T::zero(), x, T::zero(), T::one(), y, T::zero(), T::zero()])
    }
    fn scale(self, x: f32, y: f32) -> Self {
        self.transform(&[x, T::zero(), T::zero(), T::zero(), y, T::zero(), T::zero(), T::one()])
    }
    fn reflect(self, x: f32, y: f32) -> Self {
        self.transform(&[T::one(), T::zero(), T::zero(), T::zero(), T::one(), T::zero(), x, y])
    }
    fn skew(self, x: f32, y: f32) -> Self {
        self.transform(&[T::one(), x, T::zero(), y, T::one(), T::zero(), T::zero(), T::zero()])
    }
}

pub trait Transform3D {
    fn transform_3d(&self, other: &Self) -> Self;
}
