use bitflags::bitflags;

use super::*;

pub struct Polyline<T> {
    pub points: Vec<T>,
    pub closed: bool,
}

#[derive(Debug, Clone)]
pub struct Polygon<T> {
    pub vertex: Vec<Point<T>>,
    pub convex_hull: Option<bool>,
    pub self_intersect: Option<bool>,
    pub holes: Vec<Polygon<T>>,
}

impl<T> Polygon<T> {
    pub fn new<P>(points: Vec<P>) -> Self
    where
        Point<T>: From<P>,
    {
        Self { vertex: points.into_iter().map(|p| p.into()).collect(), convex_hull: None, self_intersect: None, holes: vec![] }
    }
}

impl<T: Clone> From<&Triangle<T>> for Polygon<T> {
    fn from(v: &Triangle<T>) -> Self {
        Self { vertex: v.vertex.to_vec() }
    }
}
