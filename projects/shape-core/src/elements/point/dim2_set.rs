use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Multipoint<T> {
    pub points: Vec<Point<T>>,
}

impl<T> Multipoint<T> {
    pub fn new(points: &[Point<T>]) -> Self {
        Self { points: points.to_vec() }
    }
}

impl<T, P> FromIterator<P> for Multipoint<T>
where
    P: Into<Point<T>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
    {
        Self { points: iter.into_iter().map(|p| p.into()).collect() }
    }
}
