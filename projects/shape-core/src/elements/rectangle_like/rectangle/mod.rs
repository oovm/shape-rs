use super::*;
mod constructors;

use std::ops::Sub;

/// A rectangle without rotated.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rectangle<T> {
    /// origin x point of the rectangle
    pub x: T,
    /// origin y point of the rectangle
    pub y: T,
    /// width of the rectangle
    pub w: T,
    /// height of the rectangle
    pub h: T,
}

impl<T> Rectangle<T> {
    pub fn origin(&self) -> Point<&T> {
        Point { x: &self.x, y: &self.y }
    }
    pub fn center(&self) -> Point<T>
    where
        T: Clone + One + Add<Output = T> + Sub<Output = T> + Div<Output = T>,
    {
        let half_width = self.w.clone() / two::<T>();
        let half_height = self.h.clone() / two::<T>();
        let x = self.x.clone() + half_width.clone();
        let y = self.y.clone() + half_height.clone();
        Point { x, y }
    }
    pub fn ref_inner(&self) -> Rectangle<&T> {
        Rectangle { x: &self.x, y: &self.y, w: &self.w, h: &self.h }
    }
}
