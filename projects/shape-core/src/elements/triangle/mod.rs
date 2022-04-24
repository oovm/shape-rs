mod convert;
use super::*;

#[derive(Debug, Clone)]
pub struct Triangle<T> {
    pub vertex: [Point<T>; 3],
}

impl<T> Triangle<T> {
    pub fn new<P>(vertex: [P; 3]) -> Self
    where
        Point<T>: From<P>,
    {
        let [a, b, c] = vertex;
        Self { vertex: [a.into(), b.into(), c.into()] }
    }
}

impl<T: Clone> Triangle<T> {
    pub fn is_valid(&self) -> bool {
        let (ab, ac, _) = self.edges();
        ab == ac
    }
    pub fn is_congruent(&self) -> bool {
        true
    }
    pub fn is_isosceles(&self) -> bool {
        true
    }
    #[inline]
    fn edges(&self) -> (Vector<T>, Vector<T>, Vector<T>) {
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
}
