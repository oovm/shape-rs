use super::*;

impl<T> Default for Rectangle<T> {
    fn default() -> Self {
        Self { anchor: Point::default(), side: (two(), one()) }
    }
}

impl<T> Rectangle<T>
where
    T: Clone,
{
    pub fn new(center: Point<T>, side: (T, T)) -> Self {
        Self { anchor: center, side }
    }
    pub fn from_center(anchor: Point<T>, side: (T, T)) -> Self {
        let (a, b) = side.clone();
        Self { anchor: anchor + (a / two(), b / two()), side }
    }
    pub fn from_diagonal_points(p1: Point<T>, p2: Point<T>) -> Self {
        let anchor = p1.clone();
        let (a, b) = (p1.x - p2.x, p1.y - p2.y);
        Self { anchor: Point::new(anchor.x + a / two(), anchor.y + b / two()), side: (a, b) }
    }
    pub fn from_3_points(p1: Point<T>, p2: Point<T>, p3: Point<T>) -> Self {
        let (a, b) = (p1.x.clone() - p2.x, p1.y.clone() - p2.y);
        let (c, d) = (p1.x.clone() - p3.x, p1.y.clone() - p3.y);
        Self { anchor: (p1.x + a / two(), p1.y + b / two()), side: (a, b) }
    }
}
