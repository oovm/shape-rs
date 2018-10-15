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
    /// origin x points of the rectangle
    pub min: Point<T>,
    /// origin y points of the rectangle
    pub max: Point<T>,
}

impl<T> Shape2D for Rectangle<T>
where
    T: Signed + Clone + PartialOrd,
{
    type Value = T;

    fn is_valid(&self) -> bool {
        self.max.x > self.min.x && self.max.y > self.min.y
    }

    fn normalize(&mut self) -> bool {
        todo!()
    }

    fn boundary(&self) -> Rectangle<Self::Value> {
        self.clone()
    }

    fn vertices(&self, _: usize) -> impl Iterator<Item = Point<Self::Value>> + '_ {
        from_generator(move || {
            yield self.min.clone();
            yield Point { x: self.max.x.clone(), y: self.min.y.clone() };
            yield self.max.clone();
            yield Point { x: self.min.x.clone(), y: self.max.y.clone() };
        })
    }

    fn edges(&self, _: usize) -> impl Iterator<Item = Line<Self::Value>> + '_ {
        let mut start = self.min.clone();
        let mut end = Point { x: self.max.x.clone(), y: self.min.y.clone() };
        from_generator(move || {
            yield Line::new(start.clone(), end.clone());
            start = end.clone();
            end = self.max.clone();
            yield Line::new(start.clone(), end.clone());
            start = end.clone();
            end = Point { x: self.min.x.clone(), y: self.max.y.clone() };
            yield Line::new(start.clone(), end.clone());
            start = end.clone();
            end = self.min.clone();
            yield Line::new(start.clone(), end.clone());
        })
    }
}

impl<T> Rectangle<T> {
    /// Get the origin points of the rectangle_2d
    pub fn origin(&self) -> Point<&T> {
        self.min.ref_inner()
    }
    pub fn width(&self) -> T
    where
        T: Clone + Sub<Output = T>,
    {
        self.max.x.clone() - self.min.x.clone()
    }
    pub fn height(&self) -> T
    where
        T: Clone + Sub<Output = T>,
    {
        self.max.y.clone() - self.min.y.clone()
    }
    /// Get the center points of the rectangle_2d
    pub fn center(&self) -> Point<T>
    where
        T: Clone + One + Add<Output = T> + Sub<Output = T> + Div<Output = T>,
    {
        Point { x: (self.min.x.clone() + self.max.x.clone()) / two(), y: (self.min.y.clone() + self.max.y.clone()) / two() }
    }
    /// Move reference to the inner value
    pub fn ref_inner(&self) -> Rectangle<&T> {
        Rectangle { min: self.min.ref_inner(), max: self.max.ref_inner() }
    }
}
