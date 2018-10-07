use super::*;
mod convert;
pub mod point_2d;
pub mod point_2d_set;
pub mod point_3d;
mod proj;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Point4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
