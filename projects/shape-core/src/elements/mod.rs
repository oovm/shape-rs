pub use self::{
    curves::{BSplineCurve, BezierCurve},
    ellipses::{Ball, Circle, Circle3D, Ellipse, Ellipse3D, Ellipsoid},
    lines::{
        line_2d::{Line, Vector},
        line_3d::{Line3D, Vector3D},
    },
    points::{point_2d::Point, point_2d_set::PointSet, point_3d::Point3D, Point4D},
    polygons::{normal_2d::Polygon, normal_3d::Polygon3D, Polyline, RegularPolygon},
    rectangles::{rectangle_2d::Rectangle, rectangle_3d::Cuboid, square_2d::Square, Parallelogram},
    triangles::{Triangle, TriangleIndex},
};
use num_traits::Signed;

use crate::traits::Shape2D;
use distantia::{EuclideanDistance, ManhattanDistance};
use itertools::Itertools;
use num_traits::{real::Real, Float, FloatConst, Num, NumOps, One, Zero};
use projective::Projective;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Div, Neg, Sub},
};
mod curves;
mod ellipses;
mod lines;
mod mesh;
mod points;
mod polygons;
mod rectangles;
mod triangles;

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
