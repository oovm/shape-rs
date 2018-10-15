use super::*;
mod display;

impl<T> Point3D<T> {
    /// Construct new points
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn ref_inner(&self) -> Point3D<&T> {
        Point3D { x: &self.x, y: &self.y, z: &self.z }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
