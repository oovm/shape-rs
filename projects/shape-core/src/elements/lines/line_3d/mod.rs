use super::*;

mod constructors;
/// A lines segment of finite length in 3D space, determined by a starting points and an ending points
#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Line3D<T> {
    /// Start points of the lines segment in 3D space.
    pub s: Point3D<T>,
    /// End points of the lines segment in 3D space.
    pub e: Point3D<T>,
}

#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vector3D<T> {
    pub dx: T,
    pub dy: T,
    pub dz: T,
}

impl<T> Line3D<T> {
    /// Construct new lines
    pub fn new<P>(start: P, end: P) -> Self
    where
        Point3D<T>: From<P>,
    {
        Self { s: start.into(), e: end.into() }
    }
}

impl<T: Real> Line3D<T> {
    pub fn quantile_point(&self, p: usize, q: usize) -> Point3D<T> {
        let _ = (p, q);
        todo!()
    }
    pub fn middle_point(&self) -> Point3D<T> {
        let mx = (self.s.x + self.e.x) / two();
        let my = (self.s.y + self.e.y) / two();
        let mz = (self.s.z + self.e.z) / two();
        Point3D { x: mx, y: my, z: mz }
    }
}
