use super::*;
use std::ops::Sub;

impl<T> Default for Point<T>
where
    T: Zero,
{
    fn default() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }
}

impl<T> From<&Self> for Point<T>
where
    T: Clone,
{
    fn from(value: &Self) -> Self {
        Self { x: value.x.clone(), y: value.y.clone() }
    }
}

impl<T> From<(T, T)> for Point<T>
where
    T: Clone,
{
    fn from(p: (T, T)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl<T, V> Add<V> for Point<T>
where
    Point<T>: From<V>,
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        let rhs: Self = rhs.into();
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T, V> Sub<V> for Point<T>
where
    Point<T>: From<V>,
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs: Self = rhs.into();
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
