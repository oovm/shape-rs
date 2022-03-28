use crate::elements::point::Point;

impl<T> Point<T> {
    /// Construct new point
    pub fn from_x_y(x: T, y: T) -> Self {
        Self { x, y }
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
