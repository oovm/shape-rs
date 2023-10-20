use super::*;

pub mod point_2d;
pub mod point_2d_set;
pub mod point_3d;

#[cfg_attr(feature = "serde", repr(C), derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point4D<T> {
    /// x-coordinate of a 4D point
    pub x: T,
    /// y-coordinate of a 4D point
    pub y: T,
    /// z-coordinate of a 4D point
    pub z: T,
    /// w-coordinate of a 4D point
    pub w: T,
}
