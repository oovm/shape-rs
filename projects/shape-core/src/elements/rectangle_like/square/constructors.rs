use super::*;

impl<T> Debug for Square<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let origin = Point { x: &self.x, y: &self.y };
        f.debug_struct("Square").field("origin", &origin).field("side", &self.s).finish()
    }
}

impl<T: Display> Display for Square<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Square(x: {}, y: {}, s: {})", self.x, self.y, self.s)
    }
}

impl<T> Square<T>
where
    T: Num + Clone,
{
    /// Create a square from the origin point.
    pub fn new(x: T, y: T, side: T) -> Self {
        Self { x, y, s: side }
    }
    /// Create a square from the anchor point.
    pub fn from_anchor<P>(anchor: P, side: T) -> Self
    where
        P: Into<Point<T>>,
    {
        let Point { x, y } = anchor.into();
        Self { x, y, s: side }
    }
    /// Create a square from the center point.
    pub fn from_center<P>(center: P, side: T) -> Self
    where
        P: Into<Point<T>>,
    {
        let Point { x: x0, y: y0 } = center.into();
        let Point { x, y } = Point::new(x0 - side.clone() / two(), y0 - side.clone() / two());
        Self { x, y, s: side }
    }
}
