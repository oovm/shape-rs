use super::*;

impl Debug for MeshTriangleIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MeshTriangleIndex").field(&self.a).field(&self.b).field(&self.c).finish()
    }
}

impl Display for MeshTriangleIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.a).field(&self.b).field(&self.c).finish()
    }
}
