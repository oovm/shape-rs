use super::*;
use num_traits::{Num, One};

impl<T> Default for Line<T>
where
    T: Zero + One,
{
    #[inline(always)]
    fn default() -> Self {
        Self { start: Point { x: zero(), y: zero() }, end: Point { x: one(), y: zero() } }
    }
}

impl<T> Line<T> {
    #[inline(always)]
    pub fn new<P>(start: P, end: P) -> Self
    where
        Point<T>: From<P>,
    {
        Self { start: start.into(), end: end.into() }
    }
}

impl<T> Line<T>
where
    T: Clone + Float,
{
    pub fn length(&self) -> T {
        (self.end.clone() - self.start.clone()).norm()
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

// impl Line<T> {
//     pub fn is_empty(&self, ctx: &StyleResolver) -> bool {
//         let length = || self.start == self.end;
//         let width = || self.width.unwrap_or(ctx.line_width()) <= 0.0;
//         length() || width()
//     }
// }
