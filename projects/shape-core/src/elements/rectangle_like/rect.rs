use super::*;

impl<T> Rectangle<T>
where
    T: Clone + Num,
{
    pub fn new<P>(anchor: P, side: (T, T)) -> Self
    where
        P: Into<Point<T>>,
    {
        Self { anchor: anchor.into(), side }
    }
    pub fn from_center(anchor: Point<T>, side: (T, T)) -> Self {
        todo!()
    }
    pub fn from_diagonal_points(p1: Point<T>, p2: Point<T>) -> Rectangle<T> {
        let size = p2.clone() - p1.clone();
        Self { anchor: p1, side: (size.x, size.y) }
    }
}
