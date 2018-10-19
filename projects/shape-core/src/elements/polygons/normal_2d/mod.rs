use super::*;

mod constructors;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Polygon<T> {
    pub points_set: PointSet<T>,
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
