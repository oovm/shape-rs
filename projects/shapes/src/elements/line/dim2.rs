use super::*;

impl<T> Line<T> {
    pub fn from_2_points(start: Point<T>, end: Point<T>) -> Self {
        Self { start, end }
    }
}

// impl Line<T> {
//     pub fn is_empty(&self, ctx: &StyleResolver) -> bool {
//         let length = || self.start == self.end;
//         let width = || self.width.unwrap_or(ctx.line_width()) <= 0.0;
//         length() || width()
//     }
// }
