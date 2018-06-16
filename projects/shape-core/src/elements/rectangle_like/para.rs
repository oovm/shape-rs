use super::*;

impl<T> Parallelogram<T> {
    pub fn new(anchor: Point<T>, side: (Line<T>, Line<T>)) -> Self {
        Self { anchor, side }
    }
}
