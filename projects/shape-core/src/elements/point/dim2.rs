use super::*;

impl<T> Point<T> {
    /// Construct new point
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
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

// impl Point<T> {
//     /// Distance between two points.
//     pub fn distance_to(&self, other: &Self) -> f32 {
//         let dx = self.x - other.x;
//         let dy = self.y - other.y;
//         (dx * dx + dy * dy).sqrt()
//     }
// }
