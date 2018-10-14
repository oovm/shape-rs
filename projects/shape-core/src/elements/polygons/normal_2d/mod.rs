use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polygon<T> {
    pub points_set: PointSet<T>,
}

impl<T> Polygon<T> {
    pub fn new<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point<T>>,
    {
        Self { points_set: points.into_iter().collect() }
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

impl<T> Shape2D for Polygon<T>
where
    T: NumOps + PartialOrd + Clone,
{
    type Value = T;

    /// no collinear, common points
    fn is_valid(&self) -> bool {
        todo!()
    }

    fn boundary(&self) -> Rectangle<Self::Value> {
        self.points_set.boundary()
    }

    fn vertices(&self, sample: usize) -> impl Iterator<Item = Point<Self::Value>> + '_ {
        self.points_set.vertices(sample)
    }

    fn edges(&self, sample: usize) -> impl Iterator<Item = Line<Self::Value>> + '_ {
        self.points_set.edges(sample)
    }
}
