use super::*;
use crate::elements::points::point_2d_set::PointsIterator;
use std::{marker::PhantomData, vec::IntoIter};

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
    type VertexIterator<'a>
    where
        T: 'a,
    = PointsIterator<'a, T>;
    type LineIterator<'a>
    where
        T: 'a,
    = IntoIter<Line<T>>;

    /// no collinear, common points
    fn is_valid(&self) -> bool {
        todo!()
    }

    fn boundary(&self) -> Rectangle<Self::Value> {
        self.points_set.boundary()
    }

    fn vertices<'a>(&'a self, sample: usize) -> Self::VertexIterator<'a> {
        self.points_set.vertices(sample)
    }

    fn edges<'a>(&'a self, sample: usize) -> Self::LineIterator<'a> {
        self.points_set.edges(sample)
    }
}
