use super::*;
use num_traits::{Num, One};
mod convert;

/// represents an infinitely long line_like segment
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vector<T> {
    pub dx: T,
    pub dy: T,
}

/// A line_like segment of finite length, determined by a starting point and an ending point.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Line<T> {
    /// Start point of the line_like segment.
    pub s: Point<T>,
    /// End point of the line_like segment.
    pub e: Point<T>,
}

impl<T> Line<T>
where
    T: Clone + Num,
{
    pub fn length(&self) -> T
    where
        T: Real,
    {
        self.s.euclidean_distance(&self.e)
    }
    #[inline(always)]
    pub fn as_vector(&self) -> Vector<T> {
        let new = self.e.clone() - &self.s;
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
