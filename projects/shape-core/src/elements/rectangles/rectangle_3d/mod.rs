use super::*;

/// A Cuboid is a
///
/// # Notice
///
/// The constructor will not check the legality of the parameters, call [`is_valid`] to ensure that the rectangle_2d is legal.
///
/// # Examples
///
/// ```
/// # use shape_core::Square;
/// let rect = Square::new(0.0, 0.0, 1.0);
/// ```
pub struct Cuboid<T> {
    /// The origin x of the cuboid.
    pub x: T,
    /// The origin y of the cuboid.
    pub y: T,
    /// The origin z of the cuboid.
    pub z: T,
    /// The width of the cuboid.
    pub w: T,
    /// The height of the cuboid.
    pub h: T,
    /// The length of the cuboid.
    pub l: T,
}

impl<T: Debug> Debug for Cuboid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cuboid")
            .field("origin", &self.origin())
            .field("width", &self.w)
            .field("height", &self.h)
            .field("length", &self.l)
            .finish()
    }
}

impl<T> Cuboid<T> {
    /// Create a cuboid from 6 properties
    pub fn origin(&self) -> Point3D<&T> {
        Point3D { x: &self.x, y: &self.y, z: &self.z }
    }
}
