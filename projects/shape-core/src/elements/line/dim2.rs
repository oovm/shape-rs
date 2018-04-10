use std::ops::{Mul, Sub};

use super::*;

impl<T> Line<T> {
    pub fn from_2_points(start: Point<T>, end: Point<T>) -> Self {
        Self { start, end }
    }
}

impl<T> Point<T> {
    /// ```math
    /// \vec{PA}\times\vec{PB} = (a_x-b_x)*(p_y-b_y)-(p_x-b_x)*(a_y-b_y)
    /// ```
    pub fn cross_dot(&self, a: &Self, b: &Self) -> T
    where
        T: Clone + PartialOrd,
        T: Sub<Output = T> + Mul<Output = T>,
    {
        let p = (b.x.clone() - a.x.clone()) * (self.y.clone() - b.y.clone());
        let q = (b.y.clone() - a.y.clone()) * (self.x.clone() - b.x.clone());
        p - q
    }
}

// impl Line<T> {
//     pub fn is_empty(&self, ctx: &StyleResolver) -> bool {
//         let length = || self.start == self.end;
//         let width = || self.width.unwrap_or(ctx.line_width()) <= 0.0;
//         length() || width()
//     }
// }
