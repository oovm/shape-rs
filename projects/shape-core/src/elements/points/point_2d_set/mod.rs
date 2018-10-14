use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PointSet<T> {
    pub points: Vec<Point<T>>,
}

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

impl<T> Shape2D for PointSet<T>
where
    T: NumOps + PartialOrd + Clone,
{
    type Value = T;

    fn is_valid(&self) -> bool {
        self.points.len() > 0
    }

    fn boundary(&self) -> Rectangle<T> {
        debug_assert!(!self.points.is_empty(), "At least one points is required to compute the boundary");
        let first = unsafe { self.points.get_unchecked(0) };
        let mut min = first.clone();
        let mut max = first.clone();
        for p in self.points.iter().skip(1) {
            if p.x < min.x {
                min.x = p.x.clone();
            }
            if p.y < min.y {
                min.y = p.y.clone();
            }
            if p.x > max.x {
                max.x = p.x.clone();
            }
            if p.y > max.y {
                max.y = p.y.clone();
            }
        }
        Rectangle::from_diagonal(min, max)
    }

    fn vertices(&self, _: usize) -> impl Iterator<Item = Point<Self::Value>> + '_ {
        self.points.iter().map(|p| p.clone())
    }

    /// The set of points does not contain any edges
    fn edges(&self, _: usize) -> impl Iterator<Item = Line<Self::Value>> + '_ {
        [].into_iter()
    }
}
