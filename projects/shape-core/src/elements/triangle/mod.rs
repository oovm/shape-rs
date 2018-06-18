use crate::Point;

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
        Self { vertex: [Point::from(a), Point::from(b), Point::from(c)] }
    }
    pub fn area(&self) -> T {
        todo!()
    }
}
