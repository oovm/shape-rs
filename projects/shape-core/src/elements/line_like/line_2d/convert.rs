use super::*;

impl<T> Default for Line<T>
where
    T: Zero + One,
{
    #[inline(always)]
    fn default() -> Self {
        Self { s: Point { x: zero(), y: zero() }, e: Point { x: one(), y: zero() } }
    }
}
impl<T> Line<T> {
    #[inline(always)]
    pub fn new<P>(start: P, end: P) -> Self
    where
        Point<T>: From<P>,
    {
        Self { s: start.into(), e: end.into() }
    }
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

impl<T> Debug for Line<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line").field("start", &self.s).field("end", &self.e).finish()
    }
}
impl<T> Display for Line<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line({:} → {:})", self.s, self.e)
    }
}
