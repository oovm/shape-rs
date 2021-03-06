pub use self::{
    curve::{BSplineCurve, BezierCurve},
    ellipse_like::{Ball, Circle, Circle3D, Ellipse, Ellipse3D, Ellipsoid},
    line::{Line, Line3D, Vector, Vector3D},
    point::{Point, Point3D},
    polygon_like::{CirclePoints, Polygon, Polyline, RegularPolygon},
    rectangle_like::{Parallelogram, Rectangle, Square},
    triangle::Triangle,
};
use crate::Distance;
use num_traits::{real::Real, Float, FloatConst, Num, One, Zero};
use projective::Projective;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div};

mod curve;
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
fn four<T>() -> T
where
    T: One + Add<Output = T>,
{
    T::one() + T::one() + T::one() + T::one()
}

#[inline(always)]
fn pi<T: FloatConst>() -> T {
    T::PI()
}

#[inline(always)]
fn half<T>(x: T) -> T
where
    T: One + Add<Output = T> + Div<Output = T>,
{
    x.div(two())
}

#[inline(always)]
fn double<T>(x: &T) -> T
where
    T: Clone + Add<Output = T>,
{
    x.clone() + x.clone()
}

#[inline(always)]
fn two_pi<T>() -> T
where
    T: One + FloatConst + Add<Output = T>,
{
    two::<T>() * pi()
}
