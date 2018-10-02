use super::*;

impl<T> Polygon3D<T> {
    pub fn new<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point3D<T>>,
    {
        Self { vertex: points.into_iter().collect() }
    }
    pub fn edges(&self) -> impl Iterator<Item = Line3D<&T>> {
        debug_assert!(self.vertex.len() >= 2, "Polygon must have at least two points");
        self.vertex
            .iter()
            .cycle()
            .take(self.vertex.len() + 1)
            .tuple_windows()
            .map(|(a, b)| Line3D { start: a.as_ref(), end: b.as_ref() })
    }
}
