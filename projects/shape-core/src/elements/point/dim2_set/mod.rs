use super::*;
use crate::Shape2D;
use num_traits::NumOps;
use std::{collections::BTreeSet, iter::from_generator};

#[derive(Debug, Clone, PartialEq)]
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
        debug_assert!(!self.points.is_empty(), "At least one point is required to compute the boundary");
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
        Rectangle::from_diagonal_points(min, max)
    }

    fn vertices<'i, I>(&'i self) -> I
    where
        I: Iterator<Item = Point<&'i T>>,
    {
        todo!()
    }

    fn edges<'i, I>(&'i self) -> I
    where
        I: Iterator<Item = Line<&'i T>>,
        T: 'i,
    {
        todo!()
    }
}
