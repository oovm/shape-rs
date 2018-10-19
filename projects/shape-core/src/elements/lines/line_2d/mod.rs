use super::*;
use num_traits::{Num, One};
mod constructors;
mod convert;

/// represents an infinitely long lines segment
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vector<T> {
    /// x component of the vector
    pub dx: T,
    /// y component of the vector
    pub dy: T,
}

/// A lines segment of finite length, determined by a starting points and an ending points.
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Line<T> {
    /// Start points of the lines segment.
    pub s: Point<T>,
    /// End points of the lines segment.
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
