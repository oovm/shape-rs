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
    pub fn is_empty(&self) -> bool {
        true
    }
    pub fn is_congruent(&self) -> bool {
        true
    }
    pub fn is_isosceles(&self) -> bool {
        true
    }
    pub fn area(&self) -> T {
        // Det[{{x1, y1, 1}, {x2, y2, 1}, {x3, y3, 1}}]/2
        todo!()
    }
}
