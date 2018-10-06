use super::*;

impl<T> Line3D<T> {
    /// Construct new line
    pub fn new<P>(start: P, end: P) -> Self
    where
        Point3D<T>: From<P>,
    {
        Self { start: start.into(), end: end.into() }
    }
}

impl<T: Real> Line3D<T> {
    pub fn quantile_point(&self, p: usize, q: usize) -> Point3D<T> {
        todo!()
    }
    pub fn middle_point(&self) -> Point3D<T> {
        let mx = (self.start.x + self.end.x) / two();
        let my = (self.start.y + self.end.y) / two();
        let mz = (self.start.z + self.end.z) / two();
        Point3D { x: mx, y: my, z: mz }
    }
}
