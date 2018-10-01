mod convert;

use super::*;

#[derive(Debug, Clone)]
pub struct Triangle<T> {
    pub vertex: [Point<T>; 3],
}

impl<T> Triangle<T> {
    pub fn new<P>(a: P, b: P, c: P) -> Self
    where
        Point<T>: From<P>,
    {
        Self { vertex: [a.into(), b.into(), c.into()] }
    }
    pub fn from_mesh(vertexes: &[Point<T>], index: MeshTriangleIndex) -> Self
    where
        T: Clone,
    {
        debug_assert!(index.max() < vertexes.len(), "index {:?} out of range {}", index, vertexes.len());
        // SAFETY: the debug_assert! above ensures that the index is in range
        unsafe {
            Self {
                vertex: [
                    Point::from(vertexes.get_unchecked(index.indexes[0])),
                    Point::from(vertexes.get_unchecked(index.indexes[1])),
                    Point::from(vertexes.get_unchecked(index.indexes[2])),
                ],
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
        let det1 = self.vertex[0].x.clone() * self.vertex[1].y.clone() - self.vertex[1].x.clone() * self.vertex[0].y.clone();
        // x1 y2 - x2 y1
        let det2 = self.vertex[1].x.clone() * self.vertex[2].y.clone() - self.vertex[2].x.clone() * self.vertex[1].y.clone();
        // x2 y0 - x0 y2
        let det3 = self.vertex[2].x.clone() * self.vertex[0].y.clone() - self.vertex[0].x.clone() * self.vertex[2].y.clone();
        (det1 + det2 + det3) / two()
    }
    /// Get the inscribed circle of the triangle
    pub fn inscribed_circle(&self) -> Circle<T> {
        todo!()
    }
    /// Get the circumscribed circle of the triangle.
    pub fn circumscribed_circle(&self) -> Circle<T> {
        Circle::from_3_points(&self.vertex[0], &self.vertex[1], &self.vertex[2])
    }
    pub fn edges(&self) -> (Line<T>, Line<T>, Line<T>) {
        let ab = Line::new(&self.vertex[0], &self.vertex[1]);
        let ac = Line::new(&self.vertex[0], &self.vertex[2]);
        let bc = Line::new(&self.vertex[1], &self.vertex[2]);
        (ab, ac, bc)
    }
}
