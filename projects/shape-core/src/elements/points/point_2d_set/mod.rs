use super::*;
mod constructors;

#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PointSet<T> {
    pub points: Vec<Point<T>>,
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
        Rectangle::from_min_max(min, max)
    }

    fn vertices(&self, _: usize) -> impl Iterator<Item = Point<Self::Value>> + '_ {
        self.points.iter().map(|p| p.clone())
    }

    /// The set of points does not contain any edges
    fn edges(&self, _: usize) -> impl Iterator<Item = Line<Self::Value>> + '_ {
        [].into_iter()
    }
}
