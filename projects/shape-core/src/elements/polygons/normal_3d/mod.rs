use super::*;
mod constructors;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Polygon3D<T> {
    pub vertex: Vec<Point3D<T>>,
}
