use super::*;

mod convert;
mod dim2;
mod dim3;
mod regular;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegularPolygon<T> {
    pub sides: usize,
    pub center: Point<T>,
    pub radius: T,
    pub rotate: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CirclePoints<T> {
    ty: PhantomData<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polyline<T> {
    pub points: Vec<Point<T>>,
    pub closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polygon<T> {
    pub vertex: Vec<Point<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PolygonIndex {
    pub vertex: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Polygon3D<T> {
    pub vertex: Vec<Point3D<T>>,
}
