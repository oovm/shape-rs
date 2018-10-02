use super::*;

impl Neg for MeshTriangleIndex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { a: self.a, b: self.c, c: self.b }
    }
}

impl MeshTriangleIndex {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }

    pub fn min(&self) -> usize {
        self.a.min(self.b).min(self.c)
    }
    /// Returns the maximum index in the triangle.
    ///
    /// Used to check if the triangle index is valid
    pub fn max(&self) -> usize {
        self.a.max(self.b).max(self.c)
    }
}
