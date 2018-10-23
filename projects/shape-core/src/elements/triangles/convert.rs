use super::*;
use crate::utils::{max3, min3};
use std::vec::IntoIter;

impl<T, P> TryFrom<(P, P, P)> for Triangle<T>
where
    Point<T>: From<P>,
    T: Num + Clone + PartialOrd,
{
    type Error = ();

    fn try_from(value: (P, P, P)) -> Result<Self, Self::Error> {
        let tri = Self { a: value.0.into(), b: value.1.into(), c: value.2.into() };
        match tri.is_valid() {
            true => Ok(tri),
            false => Err(()),
        }
    }
}

impl<T> TryFrom<&Polygon<T>> for Triangle<T>
where
    T: Clone,
{
    type Error = ();

    fn try_from(value: &Polygon<T>) -> Result<Self, Self::Error> {
        match value.points_set.points.as_slice() {
            [a, b, c] => Ok(Triangle { a: a.clone(), b: b.clone(), c: c.clone() }),
            _ => Err(()),
        }
    }
}

impl<T> Shape2D for Triangle<T>
where
    T: Clone + PartialOrd + Num,
{
    type Value = T;
    type VertexIterator<'a>

    = IntoIter<Point<T>>where
        T: 'a;
    type LineIterator<'a>

    = IntoIter<Line<T>>where
        T: 'a;

    fn is_valid(&self) -> bool {
        let a = Vector::from_2_points(self.a.clone(), self.b.clone());
        let b = Vector::from_2_points(self.b.clone(), self.c.clone());
        a.dx * b.dy < a.dy * b.dx
    }

    fn boundary(&self) -> Rectangle<Self::Value> {
        let min_x = min3(&self.a.x, &self.b.x, &self.c.x).clone();
        let min_y = min3(&self.a.y, &self.b.y, &self.c.y).clone();
        let max_x = max3(&self.a.x, &self.b.x, &self.c.x).clone();
        let max_y = max3(&self.a.y, &self.b.y, &self.c.y).clone();
        Rectangle::from_min_max(Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    fn vertices<'a>(&'a self, _: usize) -> Self::VertexIterator<'a> {
        vec![self.a.clone(), self.b.clone(), self.c.clone()].into_iter()
    }

    fn edges<'a>(&'a self, _: usize) -> Self::LineIterator<'a> {
        vec![
            Line::new(self.a.clone(), self.b.clone()),
            Line::new(self.b.clone(), self.c.clone()),
            Line::new(self.c.clone(), self.a.clone()),
        ]
        .into_iter()
    }
}
