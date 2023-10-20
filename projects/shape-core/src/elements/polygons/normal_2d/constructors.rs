use super::*;

impl<T> Polygon<T> {
    /// Create a new polygon from container
    pub fn new<I>(points: I) -> Self
    where
        I: Iterator<Item = Point<T>>,
    {
        Self { points_set: points.collect() }
    }
}

impl<T, P> FromIterator<P> for Polygon<T>
where
    P: Into<Point<T>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
    {
        let set = PointSet::from_iter(iter);
        Self { points_set: set }
    }
}

impl<T> Debug for Polygon<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Polygon ")?;
        f.debug_set().entries(&self.points_set.points).finish()
    }
}
