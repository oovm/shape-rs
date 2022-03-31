use super::*;
use num_traits::Num;

pub struct Parallelogram<T> {
    anchor: Point<T>,
    side: (Line<T>, Line<T>),
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
    T: Clone,
{
    pub fn from_center(center: Point<T>, side: T, rotate: T) -> Parallelogram<T> {
        todo!()
    }
    pub fn from_anchor(anchor: Point<T>, side: T, rotate: T) -> Parallelogram<T> {
        todo!()
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
    pub fn from_diagonal_points(p1: Point<T>, p2: Point<T>) -> Parallelogram<T> {
        todo!()
    }
}
