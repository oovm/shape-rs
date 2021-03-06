use super::*;
use std::marker::PhantomData;
mod convert;
mod regular;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegularPolygon<T> {
    pub sides: usize,
    pub center: Point<T>,
    pub radius: T,
    pub rotate: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CirclePoints<T> {
    ty: PhantomData<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polyline<T> {
    pub points: Vec<Point<T>>,
    pub closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polygon<T> {
    pub vertex: Vec<Point<T>>,
}

impl<T> Polygon<T> {
    pub fn new<P>(points: Vec<P>) -> Self
    where
        Point<T>: From<P>,
    {
        Self { vertex: points.into_iter().map(|p| p.into()).collect() }
    }
}

impl<T> Polyline<T> {
    pub fn new<P>(points: Vec<P>) -> Self
    where
        Point<T>: From<P>,
    {
        Self { points: points.into_iter().map(|p| p.into()).collect(), closed: true }
    }
}
