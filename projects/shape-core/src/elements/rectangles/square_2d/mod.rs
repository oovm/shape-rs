use super::*;

mod constructors;

/// A square is a special case of a rectangle_2d, where the width and height are equal.
///
/// # Notice
///
/// The constructor will not check the legality of the parameters, call [`is_valid`] to ensure that the rectangle_2d is legal.
///
/// # Examples
///
/// ```
/// # use shape_core::Square;
/// let rect = Square::new(0.0, 0.0, 1.0);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Square<T> {
    /// The origin x points of the square_2d.
    pub x: T,
    /// The origin y points of the square_2d.
    pub y: T,
    /// The side length of the square_2d.
    pub s: T,
}

impl<T> Shape2D for Square<T>
where
    T: Signed + Clone,
{
    type Value = T;

    fn is_valid(&self) -> bool {
        self.s.is_positive()
    }
    fn normalize(&mut self) -> bool {
        todo!()
    }
    fn boundary(&self) -> Rectangle<Self::Value> {
        Rectangle { x: self.x.clone(), y: self.y.clone(), w: self.s.clone(), h: self.s.clone() }
    }

    fn vertices(&self, _: usize) -> impl Iterator<Item = Point<Self::Value>> + '_ {
        from_generator(move || {
            yield Point { x: self.x.clone(), y: self.y.clone() };
            yield Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() };
            yield Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() + self.s.clone() };
            yield Point { x: self.x.clone(), y: self.y.clone() + self.s.clone() };
        })
    }

    fn edges(&self, _: usize) -> impl Iterator<Item = Line<Self::Value>> + '_ {
        let mut start = Point { x: self.x.clone(), y: self.y.clone() };
        let mut end = Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() };
        from_generator(move || {
            yield Line::new(start.clone(), end.clone());
            start = end;
            end = Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() + self.s.clone() };
            yield Line::new(start.clone(), end.clone());
            start = end;
            end = Point { x: self.x.clone(), y: self.y.clone() + self.s.clone() };
            yield Line::new(start.clone(), end.clone());
            start = end;
            end = Point { x: self.x.clone(), y: self.y.clone() };
            yield Line::new(start, end);
        })
    }
}
