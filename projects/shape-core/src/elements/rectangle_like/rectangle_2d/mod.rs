use super::*;

mod constructors;

/// A non-rotated rectangle, used to represent AABB collision boxes.
///
/// # Notice
///
/// The constructor will not check the legality of the parameters, it is possible that two points coincide, or the side length is negative
///
/// # Examples
///
/// ```
/// # use shape_core::Rectangle;
/// let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rectangle<T> {
    /// origin x point of the rectangle
    pub x: T,
    /// origin y point of the rectangle
    pub y: T,
    /// width of the rectangle_2d
    pub w: T,
    /// height of the rectangle_2d
    pub h: T,
}

impl<T> Shape2D for Rectangle<T>
where
    T: Signed + Clone,
{
    type Value = T;

    fn is_valid(&self) -> bool {
        self.w.is_positive() && self.h.is_positive()
    }

    fn normalize(&mut self) -> bool {
        todo!()
    }

    fn boundary(&self) -> Rectangle<Self::Value> {
        self.clone()
    }

    fn vertices(&self, _: usize) -> impl Iterator<Item = Point<Self::Value>> + '_ {
        from_generator(move || {
            yield Point { x: self.x.clone(), y: self.y.clone() };
            yield Point { x: self.x.clone() + self.w.clone(), y: self.y.clone() };
            yield Point { x: self.x.clone() + self.w.clone(), y: self.y.clone() + self.h.clone() };
            yield Point { x: self.x.clone(), y: self.y.clone() + self.h.clone() };
        })
    }

    fn edges(&self, _: usize) -> impl Iterator<Item = Line<Self::Value>> + '_ {
        let mut start = Point { x: self.x.clone(), y: self.y.clone() };
        let mut end = Point { x: self.x.clone() + self.w.clone(), y: self.y.clone() };
        from_generator(move || {
            yield Line::new(start.clone(), end.clone());
            start = end;
            end = Point { x: self.x.clone() + self.w.clone(), y: self.y.clone() + self.h.clone() };
            yield Line::new(start.clone(), end.clone());
            start = end;
            end = Point { x: self.x.clone(), y: self.y.clone() + self.h.clone() };
            yield Line::new(start.clone(), end.clone());
            start = end;
            end = Point { x: self.x.clone(), y: self.y.clone() };
            yield Line::new(start, end);
        })
    }
}

impl<T> Rectangle<T> {
    /// Get the origin point of the rectangle_2d
    pub fn origin(&self) -> Point<&T> {
        Point { x: &self.x, y: &self.y }
    }
    /// Get the center point of the rectangle_2d
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
    /// Move reference to the inner value
    pub fn ref_inner(&self) -> Rectangle<&T> {
        Rectangle { x: &self.x, y: &self.y, w: &self.w, h: &self.h }
    }
}
