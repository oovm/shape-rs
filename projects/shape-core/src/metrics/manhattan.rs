use super::*;
use std::ops::{Add, Sub};

impl<T> ManhattanDistance<T, Point<T>> for Point<T>
where
    T: PartialOrd + Clone + Sub<Output = T> + Add<Output = T>,
{
    fn manhattan_distance(&self, rhs: &Point<T>) -> T {
        // avoid call abs() method, make it suitable for usize
        let dx = if self.x > rhs.x { self.x.clone() - rhs.x.clone() } else { rhs.x.clone() - self.x.clone() };
        let dy = if self.y > rhs.y { self.y.clone() - rhs.y.clone() } else { rhs.y.clone() - self.y.clone() };
        dx + dy
    }
}
