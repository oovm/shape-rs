mod convert;
mod display;
mod indexes;

use super::*;

/// A triangle_like is a polygon with three edges and three vertices. It is one of the basic shapes in geometry.
///
/// # Arguments
///
/// * `a`:
/// * `b`:
/// * `c`:
///
/// returns: Triangle<T>
///
/// # Examples
///
/// ```
/// # use shape_core::Triangle;
/// ```
#[derive(Debug, Clone)]
pub struct Triangle<T> {
    /// The 1st vertex of the triangle.
    pub a: Point<T>,
    /// The 2nd vertex of the triangle.
    pub b: Point<T>,
    /// The 3rd vertex of the triangle.
    pub c: Point<T>,
}

/// Clockwise means the front side, and counterclockwise means the back side. When rendering, only the front side is rendered by default, and the back side is invisible.
///
/// If you need double-sided display, you need to draw the reverse side at the same time, you can call !self to get the reverse side
#[derive(Copy, Clone)]
pub struct TriangleIndex {
    /// The 1st vertex in the triangle index.
    pub a: usize,
    /// The 2nd vertex in the triangle index.
    pub b: usize,
    /// The 3rd vertex in the triangle index.
    pub c: usize,
}

impl<T> Triangle<T> {
    /// Create a new triangle from three points.
    pub fn new<P>(a: P, b: P, c: P) -> Self
    where
        P: Into<Point<T>>,
    {
        Self { a: a.into(), b: b.into(), c: c.into() }
    }
    /// Create a triangle from a mesh and a triangle_like index.
    pub fn from_mesh(vertexes: &[Point<T>], index: TriangleIndex) -> Self
    where
        T: Clone,
    {
        debug_assert!(
            index.max() < vertexes.len(),
            "triangle_like index {index} out of range, must less than {}",
            vertexes.len()
        );
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
    T: Clone + AddAssign + Real,
{
    /// Returns true if the triangle is equilateral.
    pub fn is_congruent(&self) -> bool {
        true
    }
    /// Returns true if the triangle is equilateral.
    pub fn is_isosceles(&self) -> bool {
        true
    }

    /// Returns the perimeter of the triangle.
    pub fn perimeter(&self) -> T {
        let mut out = T::zero();
        for edge in self.edges(3) {
            out += edge.length();
        }
        out
    }

    /// Returns the area of the triangle_like.
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
    /// Get the inscribed circle of the triangle_like
    pub fn inscribed_circle(&self) -> Circle<T> {
        todo!()
    }
    /// Get the circumscribed circle of the triangle_like.
    pub fn circumscribed_circle(&self) -> Circle<T> {
        Circle::from_3_points(&self.a, &self.b, &self.c)
    }
}
