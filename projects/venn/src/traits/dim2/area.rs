use super::*;

impl<T> Area<T> for Point<T>
where
    T: Zero,
{
    fn area(&self) -> T {
        T::zero()
    }
}
