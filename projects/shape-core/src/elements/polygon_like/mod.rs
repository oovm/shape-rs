use super::*;

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

impl<T> From<&Triangle<T>> for Polygon<T>
where
    T: Clone,
{
    fn from(v: &Triangle<T>) -> Self {
        Self { vertex: v.vertex.to_vec() }
    }
}

impl<T> From<&Parallelogram<T>> for Polygon<T>
where
    T: Clone,
{
    fn from(v: &Parallelogram<T>) -> Self {
        todo!()
        // Self { vertex: v.vertex.to_vec() }
    }
}
