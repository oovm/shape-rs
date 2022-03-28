use super::*;
use crate::{Drawable, GraphicsShape};

impl Default for Square {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, side: 1.0, color: None }
    }
}

// impl From<(f32, f32)> for Point {
//     fn from(point: (f32, f32)) -> Self {
//         Self { x: point.0 as f32, y: point.1 as f32, ..Default::default() }
//     }
// }

impl From<Square> for Rectangle {
    fn from(v: Square) -> Self {
        Self { x: v.x, y: v.y, width: v.side, height: v.side, color: v.color }
    }
}

impl From<Square> for GraphicsShape {
    fn from(v: Square) -> Self {
        Rectangle::from(v).into()
    }
}

impl From<Square> for Drawable {
    fn from(v: Square) -> Self {
        Rectangle::from(v).into()
    }
}
