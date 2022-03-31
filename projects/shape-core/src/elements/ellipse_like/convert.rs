use super::*;

impl<T> From<&Point<T>> for Circle<T>
where
    T: One + Clone,
{
    fn from(point: &Point<T>) -> Self {
        Self { center: point.clone(), radius: T::one() }
    }
}
