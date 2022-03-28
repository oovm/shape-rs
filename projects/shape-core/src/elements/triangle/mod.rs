use crate::Point;

#[derive(Debug, Clone)]
pub struct Triangle<T> {
    pub vertex: [Point<T>; 3],
}

impl<T> Triangle<T> {
    pub fn new(vertex: [Point<T>; 3]) -> Self {
        Self { vertex }
    }
    pub fn area(&self) -> T {
        todo!()
    }
}
