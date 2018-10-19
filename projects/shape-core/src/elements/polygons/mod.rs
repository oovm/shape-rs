use super::*;

mod convert;
mod dim3;
pub mod normal_2d;
pub mod normal_3d;
mod regular;

#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegularPolygon<T> {
    pub sides: usize,
    pub center: Point<T>,
    pub radius: T,
    pub rotate: T,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Polyline<T> {
    pub points: PointSet<T>,
    pub closed: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PolygonIndex {
    pub vertex: Vec<usize>,
}
