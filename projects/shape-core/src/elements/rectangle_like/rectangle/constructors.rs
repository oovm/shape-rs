use super::*;

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

/// Constructors for rectangle
impl<T> Rectangle<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self { x, y, w: width, h: height }
    }
    /// Create a rectangle from the origin point.
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
    /// Create a rectangle from two diagonal points.
    ///
    /// # Notice
    ///
    /// The constructor will not check the legality of the parameters, it is possible that two points coincide, or the side length is negative
    ///
    /// # Examples
    ///
    /// ```
    /// # use shape_core::Rectangle;
    /// let p1 = (0.0, 0.0);
    /// let p2 = (1.0, 1.0);
    /// let rect = Rectangle::from_diagonal(p1, p2);
    /// ```
    pub fn from_diagonal<P1, P2>(p1: P1, p2: P2) -> Rectangle<T>
    where
        P1: Into<Point<T>>,
        P2: Into<Point<T>>,
        T: Clone + Sub<Output = T>,
    {
        let Point { x: x1, y: y1 } = p1.into();
        let Point { x: x2, y: y2 } = p2.into();
        let x = x1.clone();
        let y = y1.clone();
        Rectangle { x, y, w: x2 - x1, h: y2 - y1 }
    }
}
