pub use self::{
    curve::{BSplineCurve, BezierCurve},
    ellipse_like::{Ball, Circle, Circle3D, Ellipse, Ellipse3D, Ellipsoid},
    line::{Line, Line3D, Vector, Vector3D},
    point::{point_2d::Point, point_2d_set::PointSet, point_3d::Point3D, Point4D},
    polygon_like::{normal_2d::Polygon, Polygon3D, Polyline, RegularPolygon},
    rectangle_like::{rectangle::Rectangle, square::Square, Parallelogram},
    triangle_like::{Triangle, TriangleIndex},
};
use crate::traits::Shape2D;
use distantia::{EuclideanDistance, ManhattanDistance};
use itertools::Itertools;
use num_traits::{real::Real, Float, FloatConst, Num, NumOps, One, Zero};
use projective::Projective;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Div, Neg, Sub},
};
mod curve;
mod ellipse_like;
mod line;
mod mesh;
mod point;
mod polygon_like;
mod rectangle_like;
mod triangle_like;

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
