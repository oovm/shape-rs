use super::*;
mod dim2;
mod dim3;
mod proj;

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<(T, T)> for Point<T>
where
    T: Clone,
{
    fn from(p: (T, T)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl<T> From<&Point<T>> for Point<T>
where
    T: Clone,
{
    fn from(p: &Point<T>) -> Self {
        p.clone()
    }
}
