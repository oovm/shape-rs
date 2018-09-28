use super::*;
use num_traits::{Num, One};

impl<T> Default for Line<T>
where
    T: Zero + One,
{
    #[inline(always)]
    fn default() -> Self {
        Self { start: Point { x: zero(), y: zero() }, end: Point { x: one(), y: zero() } }
    }
}

impl<T> Line<T> {
    #[inline(always)]
    pub fn new<P>(start: P, end: P) -> Self
    where
        Point<T>: From<P>,
    {
        Self { start: start.into(), end: end.into() }
    }
}

impl<T> Line<T>
where
    T: Clone + Num,
{
    pub fn length(&self) -> T
    where
        T: Float,
    {
        self.start.euclidean(&self.end)
    }
    #[inline(always)]
    pub fn as_vector(&self) -> Vector<T> {
        let new = self.end.clone() - &self.start;
        Vector { dx: new.x, dy: new.y }
    }

    pub fn is_parallel(&self, rhs: &Self) -> bool {
        let a = self.as_vector();
        let b = rhs.as_vector();
        a.is_parallel(&b)
    }
    pub fn is_orthogonal(&self, rhs: &Self) -> bool {
        let a = self.as_vector();
        let b = rhs.as_vector();
        a.is_orthogonal(&b)
    }
}

impl<T> Vector<T>
where
    T: Clone + Num,
{
    pub fn from_2_points<P>(start: P, end: P) -> Self
    where
        Point<T>: From<P>,
    {
        let Point { x: x1, y: y1 } = start.into();
        let Point { x: x2, y: y2 } = end.into();
        Self { dx: x2 - x1, dy: y2 - y1 }
    }
}

impl<T> Vector<T>
where
    T: Clone + Num,
{
    pub fn is_parallel(&self, rhs: &Self) -> bool {
        let Vector { dx: x1, dy: y1 } = self.clone();
        let Vector { dx: x2, dy: y2 } = rhs.clone();
        x1 * x2 - y1 * y2 == zero()
    }
    pub fn is_orthogonal(&self, rhs: &Self) -> bool {
        let Vector { dx: x1, dy: y1 } = self.clone();
        let Vector { dx: x2, dy: y2 } = rhs.clone();
        x1 * x2 + y1 * y2 == zero()
    }
}
