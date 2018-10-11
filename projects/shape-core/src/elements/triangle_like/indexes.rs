use super::*;

impl Neg for TriangleIndex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { a: self.a, b: self.c, c: self.b }
    }
}

impl TriangleIndex {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }

    pub fn min(&self) -> usize {
        self.a.min(self.b).min(self.c)
    }
    /// Returns the maximum index in the triangle_like.
    ///
    /// Used to check if the triangle_like index is valid
    pub fn max(&self) -> usize {
        self.a.max(self.b).max(self.c)
    }
}
