use crate::{Point, Point3D};
use color_core::RGBA;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Neg,
};

mod display;
mod tri;

#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<Point3D<f32>>,
    triangles: Vec<MeshTriangleIndex>,
    uvs: Vec<[f32; 4]>,
    colors: Vec<RGBA>,
}

impl<T: Default> Default for Mesh {
    fn default() -> Self {
        Self { vertices: Vec::new(), triangles: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
    }
}

impl<T> MeshVertex<T> {
    pub fn new<P>(position: P) -> Self
    where
        P: Into<Point3D<T>>,
    {
        Self {
            position: position.into(),
            uv_tiling: Point::new(1.0, 1.0),
            uv_offset: Point::new(0.0, 0.0),
            color: RGBA::new(255, 255, 255, 255),
        }
    }
}
