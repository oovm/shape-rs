use super::*;
use std::vec::IntoIter;

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
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Square<T> {
    /// The origin x points of the square.
    pub x: T,
    /// The origin y points of the square.
    pub y: T,
    /// The side length of the square.
    pub s: T,
}

impl<T> Square<T> {
    /// Get the origin of the square.
    pub fn origin(&self) -> Point<&T>
    where
        T: Clone,
    {
        Point { x: &self.x, y: &self.y }
    }
}

impl<T> Shape2D for Square<T>
where
    T: Signed + Clone + PartialOrd,
{
    type Value = T;
    type VertexIterator<'a>
    where
        Self: 'a,
    = IntoIter<Point<T>>;
    type LineIterator<'a>
    where
        Self: 'a,
    = IntoIter<Line<T>>;

    fn is_valid(&self) -> bool {
        self.s > T::zero()
    }
    fn normalize(&mut self) -> bool {
        todo!()
    }
    fn boundary(&self) -> Rectangle<Self::Value> {
        Rectangle {
            min: Point { x: self.x.clone(), y: self.y.clone() },
            max: Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() + self.s.clone() },
        }
    }

    fn vertices<'a>(&'a self, _: usize) -> Self::VertexIterator<'a> {
        vec![
            Point { x: self.x.clone(), y: self.y.clone() },
            Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() },
            Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() + self.s.clone() },
            Point { x: self.x.clone(), y: self.y.clone() + self.s.clone() },
        ]
        .into_iter()
    }

    fn edges<'a>(&'a self, _: usize) -> Self::LineIterator<'a> {
        let mut start = Point { x: self.x.clone(), y: self.y.clone() };
        let mut end = Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() };
        let mut out = Vec::with_capacity(4);
        {
            out.push(Line::new(start.clone(), end.clone()));
            start = end;
            end = Point { x: self.x.clone() + self.s.clone(), y: self.y.clone() + self.s.clone() };
            out.push(Line::new(start.clone(), end.clone()));
            start = end;
            end = Point { x: self.x.clone(), y: self.y.clone() + self.s.clone() };
            out.push(Line::new(start.clone(), end.clone()));
            start = end;
            end = Point { x: self.x.clone(), y: self.y.clone() };
            out.push(Line::new(start, end));
        };
        out.into_iter()
    }
}
