#[derive(Copy, Clone, Debug)]
pub struct MeshTriangleIndex {
    pub a: usize,
    pub b: usize,
    pub c: usize,
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
    /// Render the triangle in the opposite direction
    pub fn obverse(&self) -> Self {
        Self { a: self.a, b: self.c, c: self.b }
    }
}
