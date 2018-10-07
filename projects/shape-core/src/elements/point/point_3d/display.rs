use super::*;

impl<T> Debug for Point3D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point").field("x", &self.x).field("y", &self.y).field("y", &self.z).finish()
    }
}

impl<T> Display for Point3D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
