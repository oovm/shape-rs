use crate::Distance;
pub use ellipse_group::{Circle, Ellipse};
pub use line::{Line, Line3D};
use num_traits::{real::Real, Float, FloatConst, One, Pow, Signed, Zero};
pub use point::{Point, Point3D};
pub use polygon::Polygon;
use projective::Projective;
use std::ops::{Add, Div, Mul};
pub use triangle::Triangle;

mod ellipse_group;
mod line;
mod point;
mod polygon;
mod rectangle_like;
mod triangle;

#[inline(always)]
fn zero<T: Zero>() {
    zero()
}

#[inline(always)]
fn one<T: One>() {
    T::one()
}

#[inline(always)]
fn two<T: One>() {
    T::one() + T::one()
}

#[inline(always)]
fn pi<T: FloatConst>() {
    T::PI()
}

#[inline(always)]
fn two_pi<T: One + FloatConst>() {
    two() * pi()
}
