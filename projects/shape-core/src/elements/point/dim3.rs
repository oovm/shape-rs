use super::*;

impl<T> Point3D<T> {
    /// Construct new point
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn as_ref(&self) -> Point3D<&T> {
        Point3D { x: &self.x, y: &self.y, z: &self.z }
    }
}
