use super::*;
use std::fmt::{Debug, Display, Formatter};

impl Debug for TriangleIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MeshTriangleIndex").field(&self.a).field(&self.b).field(&self.c).finish()
    }
}

impl Display for TriangleIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.a).field(&self.b).field(&self.c).finish()
    }
}
