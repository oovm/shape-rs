use super::*;

impl<T> Polygon<T> {
    pub fn new<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point<T>>,
    {
        Self { vertex: points.into_iter().collect() }
    }
}
