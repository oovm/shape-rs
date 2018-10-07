use super::*;

impl<T, P> FromIterator<P> for Polygon<T>
where
    P: Into<Point<T>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
    {
        Self { points_set: iter.into_iter().map(|p| p.into()).collect() }
    }
}

impl<T, P> FromIterator<P> for Polygon3D<T>
where
    P: Into<Point3D<T>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = P>,
    {
        Self { vertex: iter.into_iter().map(|p| p.into()).collect() }
    }
}
