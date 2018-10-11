use super::*;

mod constructors;

/// A square without rotated.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Square<T> {
    /// The origin x point of the square.
    pub x: T,
    /// The origin y point of the square.
    pub y: T,
    /// The side length of the square.
    pub s: T,
}
