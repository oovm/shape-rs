pub use self::{
    ellipse_like::{Ball, Circle, Circle3D, Ellipse, Ellipse3D},
    line::{Line, Line3D},
    point::{Point, Point3D},
    polygon_like::Polygon,
    rectangle_like::{Parallelogram, Rectangle, Square},
    triangle::Triangle,
};
use crate::Distance;
use num_traits::{real::Real, Float, FloatConst, Num, One, Pow, Zero};
use projective::Projective;
use std::ops::Add;

mod ellipse_like;
mod line;
mod point;
mod polygon_like;
mod rectangle_like;
mod triangle;

#[inline(always)]
fn zero<T: Zero>() -> T {
    T::zero()
}

#[inline(always)]
fn one<T: One>() -> T {
    T::one()
}

#[inline(always)]
fn two<T>() -> T
where
    T: One + Add<Output = T>,
{
    T::one() + T::one()
}

#[inline(always)]
fn pi<T: FloatConst>() -> T {
    T::PI()
}

#[inline(always)]
fn two_pi<T>() -> T
where
    T: One + FloatConst + Add<Output = T>,
{
    two::<T>() * pi()
}
