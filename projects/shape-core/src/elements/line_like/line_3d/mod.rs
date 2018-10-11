use super::*;

impl<T> Line3D<T> {
    /// Construct new line_like
    pub fn new<P>(start: P, end: P) -> Self
    where
        Point3D<T>: From<P>,
    {
        Self { s: start.into(), e: end.into() }
    }
}

impl<T: Real> Line3D<T> {
    pub fn quantile_point(&self, p: usize, q: usize) -> Point3D<T> {
        todo!()
    }
    pub fn middle_point(&self) -> Point3D<T> {
        let mx = (self.s.x + self.e.x) / two();
        let my = (self.s.y + self.e.y) / two();
        let mz = (self.s.z + self.e.z) / two();
        Point3D { x: mx, y: my, z: mz }
    }
}
