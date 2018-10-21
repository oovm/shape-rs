use super::*;
use std::vec::IntoIter;
mod constructors;

#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PointSet<T> {
    pub points: Vec<Point<T>>,
}

#[derive(Debug)]
pub struct PointsIterator<'a, T> {
    iter: std::slice::Iter<'a, Point<T>>,
}

impl<'a, T> Iterator for PointsIterator<'a, T>
where
    T: Clone,
{
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p| p.clone())
    }
}

impl<T> Shape2D for PointSet<T>
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
        Rectangle::from_min_max(min, max)
    }

    fn vertices<'a>(&'a self, _: usize) -> Self::VertexIterator<'a> {
        PointsIterator { iter: self.points.iter() }
    }

    /// The set of points does not contain any edges
    fn edges<'a>(&'a self, _: usize) -> Self::LineIterator<'a> {
        vec![].into_iter()
    }
}
