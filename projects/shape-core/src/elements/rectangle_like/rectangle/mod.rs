use super::*;

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

impl<T> Debug for Rectangle<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let origin = Point { x: &self.x, y: &self.y };
        f.debug_struct("Rectangle").field("origin", &origin).field("width", &self.w).field("height", &self.h).finish()
    }
}

impl<T: Display> Display for Rectangle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle(x: {}, y: {}, w: {}, h: {})", self.x, self.y, self.w, self.h)
    }
}

impl<T> Rectangle<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self { x, y, w: width, h: height }
    }
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
    pub fn from_center(center: Point<T>, width: T, height: T) -> Self
    where
        T: Clone + One + Add<Output = T> + Sub<Output = T> + Div<Output = T>,
    {
        let half_width = width.clone() / two::<T>();
        let half_height = height.clone() / two::<T>();
        let origin = Point { x: center.x - half_width.clone(), y: center.y - half_height.clone() };
        let w = half_width * two::<T>();
        let h = half_height * two::<T>();
        Self { x: origin.x, y: origin.y, w, h }
    }
    pub fn from_diagonal_points(p1: Point<T>, p2: Point<T>) -> Rectangle<T>
    where
        T: Clone + Sub<Output = T>,
    {
        let origin = p1.clone();
        let w = p2.x.sub(p1.x);
        let h = p2.y.sub(p1.y);
        Self { x: origin.x, y: origin.y, w, h }
    }
}

impl<T> Rectangle<T>
where
    T: Clone + Num + PartialOrd,
{
    pub fn bound_box(points: &[Point<T>]) -> Rectangle<T> {
        let mut x_min = points[0].x.clone();
        let mut x_max = points[0].x.clone();
        let mut y_min = points[0].y.clone();
        let mut y_max = points[0].y.clone();
        for p in points {
            if p.x.le(&x_min) {
                x_min = p.x.clone();
            }
            if p.x.ge(&x_max) {
                x_max = p.x.clone();
            }
            if p.y.le(&y_min) {
                y_min = p.y.clone();
            }
            if p.y.ge(&y_max) {
                y_max = p.y.clone();
            }
        }
        Rectangle::from_diagonal_points(Point::new(x_min, y_min), Point::new(x_max, y_max))
    }
}
