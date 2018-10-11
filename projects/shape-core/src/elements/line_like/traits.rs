use crate::{Point, Vector};

impl<T, P> From<P> for Vector<T>
where
    Point<T>: From<P>,
{
    fn from(value: P) -> Self {
        let value: Point<T> = value.into();
        Self { dx: value.x, dy: value.y }
    }
}
