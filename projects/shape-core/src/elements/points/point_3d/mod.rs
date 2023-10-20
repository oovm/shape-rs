use super::*;
mod display;

#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point3D<T> {
    ///x-coordinate of a 3D point
    pub x: T,
    ///y-coordinate of a 3D point
    pub y: T,
    ///z-coordinate of a 3D point
    pub z: T,
}

impl<T> Point3D<T> {
    /// Construct new points
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    /// Move pointer to inner fields
    pub fn ref_inner(&self) -> Point3D<&T> {
        Point3D { x: &self.x, y: &self.y, z: &self.z }
    }
}

