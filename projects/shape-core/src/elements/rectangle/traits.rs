use super::*;

impl Default for Rectangle {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, width: 1.0, height: 0.618 }
    }
}

// impl From<(f32, f32)> for Point {
//     fn from(point: (f32, f32)) -> Self {
//         Self { x: point.0 as f32, y: point.1 as f32, ..Default::default() }
//     }
// }

// impl From<Rectangle> for GraphicsShape {
//     fn from(v: Rectangle) -> Self {
//         Self::Rectangle(v)
//     }
// }
