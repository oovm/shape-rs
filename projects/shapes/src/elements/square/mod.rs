use super::*;
mod traits;

impl Square {
    pub fn new(x1: f32, y1: f32, side: f32) -> Self {
        Self { x: x1, y: y1, side, ..Default::default() }
    }
}
