use super::*;

impl<T> ConvexHull<T> for Point<T> {
    type Output = Polygon<T>;

    fn get_convex_hull(&self, _: Option<T>) -> Option<Self::Output> {
        None
    }
}

impl<T> ConvexHull<T> for &[Point<T>]
where
    T: PartialOrd + Clone + Signed,
{
    type Output = Polygon<T>;

    fn get_convex_hull(&self, tolerance: Option<T>) -> Option<Self::Output> {
        let v: Vec<_> = self.iter().map(|p| (p.x.clone(), p.y.clone())).collect();
        v.get_convex_hull(tolerance)
    }
}
