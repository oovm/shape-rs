use super::*;

impl<T> Line<T> {
    /// Create a new line from two point
    #[inline(always)]
    pub fn new<P>(start: P, end: P) -> Self
    where
        Point<T>: From<P>,
    {
        Self { s: start.into(), e: end.into() }
    }
    /// Create a new line from anchor point and vector
    pub fn from_anchor<S, V>(start: S, v: V) -> Self
    where
        T: Clone + Add<Output = T>,
        S: Into<Point<T>>,
        V: Into<Vector<T>>,
    {
        let s = start.into();
        let Vector { dx: vx, dy: vy } = v.into();
        let e = Point { x: s.x.clone() + vx, y: s.y.clone() + vy };
        Self { s, e }
    }
}
