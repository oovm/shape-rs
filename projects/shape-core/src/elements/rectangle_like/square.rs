use super::*;
use num_traits::Num;

pub struct Parallelogram<T> {
    pub anchor: Point<T>,
    pub side: (Line<T>, Line<T>),
}

impl<T> Parallelogram<T> {
    pub fn is_square(&self) -> bool {
        todo!()
    }
    pub fn is_rectangle(anchor: Point<T>, side: (Line<T>, Line<T>)) -> Self {
        Self { anchor, side }
    }
}

impl<T> Square<T>
where
    T: Num + Clone,
{
    pub fn from_anchor<P>(anchor: P, side: T) -> Self
    where
        Point<T>: From<P>,
    {
        Self { anchor: Point::from(anchor), side }
    }

    pub fn from_center<P>(center: P, side: T) -> Self
    where
        Point<T>: From<P>,
    {
        let center = Point::from(center);
        let anchor = Point::new(center.x - side.clone() / two(), center.y - side.clone() / two());
        Self { anchor, side }
    }
}

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
