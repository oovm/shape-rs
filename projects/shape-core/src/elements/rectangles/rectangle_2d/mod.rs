use super::*;
use crate::utils::{max2, min2};
use std::{ops::Mul, vec::IntoIter};

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
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    type VertexIterator<'a>

    = IntoIter<Point<T>>where
        T: 'a;
    type LineIterator<'a>

    = IntoIter<Line<T>>where
        T: 'a;

    /// A valid rectangle means it has a positive area.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::{Rectangle, Shape2D};
    /// let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
    /// assert!(rect.is_valid());
    /// ```
    fn is_valid(&self) -> bool {
        // can not be zero area
        self.max.x > self.min.x && self.max.y > self.min.y
    }

    fn normalize(&mut self) -> bool {
        *self = self.boundary();
        // maybe zero area after flip
        self.is_valid()
    }
    /// # SAFETY
    ///
    /// It may return a zero area rectangle if shape is not valid.
    ///
    /// It never returns a rectangle with negative area.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use shape_core::{Rectangle, Shape2D};
    /// let rect = Rectangle::new(0.0, 0.0, -1.0, -1.0);
    /// assert_eq!(rect.boundary(), Rectangle::new(0.0, 0.0, -1.0, -1.0));
    /// ```
    fn boundary(&self) -> Rectangle<Self::Value> {
        let min_x = min2(&self.min.x, &self.max.x).clone();
        let min_y = min2(&self.min.y, &self.max.y).clone();
        let max_x = max2(&self.max.x, &self.min.x).clone();
        let max_y = max2(&self.max.y, &self.min.y).clone();
        Rectangle { min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } }
    }

    /// Returns four vertices counterclockwise in the **↑Y coordinate system**
    fn vertices<'a>(&'a self, _: usize) -> Self::VertexIterator<'a> {
        // yield self.min.clone();
        // yield Point { x: self.max.x.clone(), y: self.min.y.clone() };
        // yield self.max.clone();
        // yield Point { x: self.min.x.clone(), y: self.max.y.clone() };
        vec![
            self.min.clone(),
            Point { x: self.max.x.clone(), y: self.min.y.clone() },
            self.max.clone(),
            Point { x: self.min.x.clone(), y: self.max.y.clone() },
        ]
        .into_iter()
    }

    /// Returns four edges counterclockwise in the **↑Y coordinate system**
    fn edges<'a>(&'a self, _: usize) -> Self::LineIterator<'a> {
        let mut start = self.min.clone();
        let mut end = Point { x: self.max.x.clone(), y: self.min.y.clone() };
        let mut out = Vec::with_capacity(4);
        {
            out.push(Line::new(start.clone(), end.clone()));
            start = end.clone();
            end = self.max.clone();
            out.push(Line::new(start.clone(), end.clone()));
            start = end.clone();
            end = Point { x: self.min.x.clone(), y: self.max.y.clone() };
            out.push(Line::new(start.clone(), end.clone()));
            start = end.clone();
            end = self.min.clone();
            out.push(Line::new(start.clone(), end.clone()));
        };
        out.into_iter()
    }
}

impl<T> Rectangle<T> {
    /// Get the width of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::Rectangle;
    /// let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
    /// assert_eq!(rect.width(), 1.0);
    /// ```
    pub fn width(&self) -> T
    where
        T: Clone + Sub<Output = T>,
    {
        self.max.x.clone() - self.min.x.clone()
    }

    /// Get the height of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::Rectangle;
    /// let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
    /// assert_eq!(rect.height(), 1.0);
    /// ```
    pub fn height(&self) -> T
    where
        T: Clone + Sub<Output = T>,
    {
        self.max.y.clone() - self.min.y.clone()
    }
    /// Get the origin of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::{Point, Rectangle};
    /// let rect = Rectangle::from_center((0.0, 0.0), 1.0, 1.0);
    /// assert_eq!(rect.origin(), Point::new(-2.0, -2.0));
    /// ```
    pub fn origin(&self) -> Point<T>
    where
        T: Clone,
    {
        self.min.clone()
    }
    /// Get the center point of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::{Point, Rectangle};
    /// let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
    /// assert_eq!(rect.center(), Point::new(0.5, 0.5));
    /// ```
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
    pub fn contains(&self, point: &Point<T>) -> bool
    where
        T: Clone + PartialOrd,
    {
        point.x >= self.min.x.clone()
            && point.x <= self.max.x.clone()
            && point.y >= self.min.y.clone()
            && point.y <= self.max.y.clone()
    }
    /// Check if two rectangle had overlapped
    pub fn overlaps(&self, other: &Rectangle<T>) -> bool
    where
        T: Clone + PartialOrd,
    {
        self.min.x.clone() <= other.max.x.clone()
            && self.max.x.clone() >= other.min.x.clone()
            && self.min.y.clone() <= other.max.y.clone()
            && self.max.y.clone() >= other.min.y.clone()
    }
    pub fn area(&self) -> T
    where
        T: Clone + Mul<Output = T> + Sub<Output = T>,
    {
        (self.max.x.clone() - self.min.x.clone()) * (self.max.y.clone() - self.min.y.clone())
    }
}
