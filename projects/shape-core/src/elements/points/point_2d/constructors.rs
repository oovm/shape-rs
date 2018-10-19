use super::*;

impl<T> Debug for Point<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point").field("x", &self.x).field("y", &self.y).finish()
    }
}

impl<T> Display for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Zero> Default for Point<T> {
    fn default() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }
}

impl<T> Point<T> {
    /// Construct new points
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
