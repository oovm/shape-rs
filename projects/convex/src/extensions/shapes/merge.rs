use super::*;

impl<T> ConvexHullMerge<T> for Polygon<T> {
    fn is_intersect(&self, other: &Self, tolerance: &Option<T>) -> bool {
        todo!()
    }

    fn merge_intersect(&mut self, rhs: Self, tolerance: Option<T>) -> Self {
        todo!()
    }

    fn merge_standalone(&mut self, rhs: Self, tolerance: Option<T>) -> Self {
        todo!()
    }
}
