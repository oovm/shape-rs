use super::*;

impl<T> Debug for Rectangle<T>
    where
        T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let origin = Point { x: &self.min, y: &self.max };
        f.debug_struct("Rectangle").field("origin", &origin).field("diagonal", &self.max).finish()
    }
}

impl<T: Display> Display for Rectangle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle({}, {})", self.min, self.max)
    }
}

/// Constructors for rectangle_2d
impl<T> Rectangle<T> {
    /// Create a rectangle_2d from 4 properties
    ///
    /// # Notice
    ///
    /// The constructor will not check the legality of the parameters, call [`is_valid`] to ensure that the rectangle_2d is legal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::Rectangle;
    /// let rect = Rectangle::new(0.0, 0.0, 1.0, 1.0);
    /// ```
    pub fn new(x: T, y: T, width: T, height: T) -> Self
        where
            T: Clone + Add<Output=T>,
    {
        let min = Point::new(x, y);
        Self::from_anchor(min, width, height)
    }
    /// Create a `[Rectangle]` from anchor (top left point) and 2 properties
    ///
    /// # Notice
    ///
    /// The constructor will not check the legality of the parameters, call [`is_valid`] to ensure that the rectangle_2d is legal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::Rectangle;
    /// let rect = Rectangle::from_anchor((0.0, 0.0), 1.0, 1.0);
    /// ```
    pub fn from_anchor<P>(origin: P, width: T, height: T) -> Self
        where
            T: Clone + Add<Output=T>,
            P: Into<Point<T>>,
    {
        let min = origin.into();
        let max = Point::new(min.x.clone() + width, min.y.clone() + height);
        Self { min, max }
    }
    /// Create a rectangle_2d from center and 2 properties
    ///
    /// # Notice
    ///
    /// The constructor will not check the legality of the parameters, call [`is_valid`] to ensure that the rectangle_2d is legal.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::Rectangle;
    /// let rect = Rectangle::from_center((0.0, 0.0), 1.0, 1.0);
    /// ```
    pub fn from_center<P>(center: P, width: T, height: T) -> Self
        where
            T: Clone + One + Add<Output=T> + Sub<Output=T> + Div<Output=T>,
            P: Into<Point<T>>,
    {
        let center = center.into();
        let half_width = width.clone() / (T::one() + T::one());
        let half_height = height.clone() / (T::one() + T::one());
        let min = Point::new(center.x.clone() - half_width.clone(), center.y.clone() - half_height.clone());
        let max = Point::new(center.x.clone() + half_width.clone(), center.y.clone() + half_height.clone());
        Self { min, max }
    }
    pub fn from_origin(width: T, height: T) -> Self
        where
            T: Clone + Zero +One + Add<Output=T> + Sub<Output=T> + Div<Output=T>,
    {
        let half_width = width.clone() / (T::one() + T::one());
        let half_height = height.clone() / (T::one() + T::one());
        let min = Point::new(T::zero() - half_width.clone(), T::zero() - half_height.clone());
        let max = Point::new(T::zero() + half_width.clone(), T::zero() + half_height.clone());
        Self { min, max }
    }
    /// Create a rectangle_2d from two diagonal points.
    ///
    /// # Notice
    ///
    /// The constructor will not check the legality of the parameters, it is possible that two points coincide, or the side length is negative
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::Rectangle;
    /// let rect = Rectangle::from_min_max((0.0, 0.0), (1.0, 1.0));
    /// ```
    pub fn from_min_max<P1, P2>(min: P1, max: P2) -> Rectangle<T>
        where
            P1: Into<Point<T>>,
            P2: Into<Point<T>>,
    {
        Rectangle { min: min.into(), max: max.into() }
    }
}
