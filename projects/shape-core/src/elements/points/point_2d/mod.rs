use super::*;
mod display;
mod euclidean;
mod manhattan;

/// A 2D points.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Construct new points
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn ref_inner(&self) -> Point<&T> {
        Point { x: &self.x, y: &self.y }
    }
}

impl<T> Point<T>
where
    T: Num + Clone,
{
    pub fn norm(&self) -> T
    where
        T: Float,
    {
        (self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone()).sqrt()
    }

    /// ```math
    /// \vec{PA}\times\vec{PB} = (a_x-b_x)*(p_y-b_y)-(p_x-b_x)*(a_y-b_y)
    /// ```
    pub fn cross_dot(&self, a: &Self, b: &Self) -> T {
        let p = (b.x.clone() - a.x.clone()) * (self.y.clone() - b.y.clone());
        let q = (b.y.clone() - a.y.clone()) * (self.x.clone() - b.x.clone());
        p - q
    }
}
