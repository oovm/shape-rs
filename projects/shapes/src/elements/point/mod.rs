mod dim2;
mod dim3;

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<&Point<T>> for Point<T>
where
    T: Clone,
{
    fn from(p: &Point<T>) -> Self {
        p.clone()
    }
}
