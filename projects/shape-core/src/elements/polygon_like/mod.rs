use super::*;
mod convert;

pub struct Polyline<T> {
    pub points: Vec<T>,
    pub closed: bool,
}

#[derive(Debug, Clone)]
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
