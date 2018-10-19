use super::*;

impl<T> PointSet<T> {
    pub fn new(points: &[Point<T>]) -> Self
    where
        T: Clone,
    {
        Self { points: points.to_vec() }
    }
}

impl<T, P> FromIterator<P> for PointSet<T>
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

impl<T: Debug> Debug for PointSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(&self.points).finish()
    }
}
