use super::*;

impl<T> Debug for Vector3D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector").field("x", &self.dx).field("y", &self.dy).field("z", &self.dz).finish()
    }
}

impl<T> Debug for Line3D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line").field("start", &self.s).field("end", &self.e).finish()
    }
}
