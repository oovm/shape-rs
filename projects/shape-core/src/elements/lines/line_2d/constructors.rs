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

impl<T> Debug for Vector<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector").field("x", &self.dx).field("y", &self.dy).finish()
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
