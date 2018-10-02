mod convert;
mod display;
mod indexes;

use super::*;

#[derive(Debug, Clone)]
pub struct Triangle<T> {
    pub a: Point<T>,
    pub b: Point<T>,
    pub c: Point<T>,
}

/// Clockwise means the front side, and counterclockwise means the back side. When rendering, only the front side is rendered by default, and the back side is invisible.
///
/// If you need double-sided display, you need to draw the reverse side at the same time, you can call !self to get the reverse side
#[derive(Copy, Clone)]
pub struct TriangleIndex {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl<T> Triangle<T> {
    pub fn new<P>(a: P, b: P, c: P) -> Self
    where
        Point<T>: From<P>,
    {
        Self { a: a.into(), b: b.into(), c: c.into() }
    }
    pub fn from_mesh(vertexes: &[Point<T>], index: TriangleIndex) -> Self
    where
        T: Clone,
    {
        debug_assert!(index.max() < vertexes.len(), "triangle index {index} out of range, must less than {}", vertexes.len());
        // SAFETY: the debug_assert! above ensures that the index is in range
        unsafe {
            Self {
                a: vertexes.get_unchecked(index.a).clone(),
                b: vertexes.get_unchecked(index.b).clone(),
                c: vertexes.get_unchecked(index.c).clone(),
            }
        }
    }
}

impl<T> Triangle<T>
where
    T: Clone + Real,
{
    pub fn is_valid(&self) -> bool {
        let (ab, ac, _) = self.edges();
        ab.is_parallel(&ac)
    }
    pub fn is_congruent(&self) -> bool {
        true
    }
    pub fn is_isosceles(&self) -> bool {
        true
    }
    pub fn perimeter(&self) -> T {
        todo!()
    }

    /// Returns the area of the triangle.
    pub fn area(&self) -> T {
        // Det[{{x0, y0, 1}, {x1, y1, 1}, {x2, y2, 1}}] / 2
        // x0 y1 - x1 y0
        let det1 = self.a.x.clone() * self.b.y.clone() - self.b.x.clone() * self.a.y.clone();
        // x1 y2 - x2 y1
        let det2 = self.b.x.clone() * self.c.y.clone() - self.c.x.clone() * self.b.y.clone();
        // x2 y0 - x0 y2
        let det3 = self.c.x.clone() * self.a.y.clone() - self.a.x.clone() * self.c.y.clone();
        (det1 + det2 + det3) / two()
    }
    /// Get the inscribed circle of the triangle
    pub fn inscribed_circle(&self) -> Circle<T> {
        todo!()
    }
    /// Get the circumscribed circle of the triangle.
    pub fn circumscribed_circle(&self) -> Circle<T> {
        Circle::from_3_points(&self.a, &self.b, &self.c)
    }
    pub fn edges(&self) -> (Line<T>, Line<T>, Line<T>) {
        let ab = Line::new(&self.a, &self.b);
        let ac = Line::new(&self.a, &self.c);
        let bc = Line::new(&self.b, &self.c);
        (ab, ac, bc)
    }
}
