use super::*;

mod convert;
mod dim3;
pub mod normal_2d;
mod regular;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegularPolygon<T> {
    pub sides: usize,
    pub center: Point<T>,
    pub radius: T,
    pub rotate: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polyline<T> {
    pub points: PointSet<T>,
    pub closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PolygonIndex {
    pub vertex: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polygon3D<T> {
    pub vertex: Vec<Point3D<T>>,
}
