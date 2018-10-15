use super::*;

impl<T> Polygon3D<T> {
    pub fn new(points: &[Point3D<T>]) -> Self
    where
        T: Clone,
    {
        Self { vertex: points.to_vec() }
    }
    pub fn edges(&self) -> impl Iterator<Item = Line3D<&T>> {
        debug_assert!(self.vertex.len() >= 3, "Polygon must have at least three points");
        self.vertex
            .iter()
            .cycle()
            .take(self.vertex.len() + 1)
            .tuple_windows()
            .map(|(a, b)| Line3D { s: a.ref_inner(), e: b.ref_inner() })
    }
    pub fn center(&self) -> Point3D<T>
    where
        T: Zero + One + Clone + AddAssign + Div<Output = T>,
    {
        let mut n = T::zero();
        let mut x = T::zero();
        let mut y = T::zero();
        let mut z = T::zero();
        for p in self.vertex.iter() {
            x.add_assign(p.x.clone());
            y.add_assign(p.y.clone());
            z.add_assign(p.z.clone());
            n.add_assign(T::one());
        }
        Point3D { x: x / n.clone(), y: y / n.clone(), z: z / n.clone() }
    }
}
