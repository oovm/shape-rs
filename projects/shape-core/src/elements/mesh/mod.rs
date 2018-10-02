use crate::{Point, Point3D};
use color_core::RGBA32;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Neg,
};

mod display;
mod tri;

#[derive(Copy, Clone)]
pub struct Mesh<T> {
    pub vertices: Vec<MeshVertex<T>>,
    pub triangles: Vec<MeshTriangleIndex>,
}

#[derive(Copy, Clone)]
pub struct MeshVertex<T> {
    pub position: Point3D<T>,
    pub uv_tiling: Point<f32>,
    pub uv_offset: Point<f32>,
    pub color: RGBA32,
}

/// Clockwise means the front side, and counterclockwise means the back side. When rendering, only the front side is rendered by default, and the back side is invisible.
///
/// If you need double-sided display, you need to draw the reverse side at the same time, you can call !self to get the reverse side
#[derive(Copy, Clone)]
pub struct MeshTriangleIndex {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}
