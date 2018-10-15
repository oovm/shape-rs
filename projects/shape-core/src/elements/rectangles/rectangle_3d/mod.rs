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
    pub min: Point3D<T>,
    /// The width of the cuboid.
    pub max: Point3D<T>,
}

impl<T> Debug for Cuboid<T>
where
    T: Debug + Clone + Sub<Output = T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cuboid")
            .field("origin", &self.origin())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("length", &self.length())
            .finish()
    }
}

impl<T> Cuboid<T> {
    /// Create a cuboid from 6 properties
    pub fn origin(&self) -> Point3D<&T> {
        self.min.ref_inner()
    }
    /// Delta x
    pub fn width(&self) -> T
    where
        T: Clone + Sub<Output = T>,
    {
        self.max.x.clone() - self.min.x.clone()
    }
    /// delta y
    pub fn height(&self) -> T
    where
        T: Clone + Sub<Output = T>,
    {
        self.max.y.clone() - self.min.y.clone()
    }
    /// Delta z
    pub fn length(&self) -> T
    where
        T: Clone + Sub<Output = T>,
    {
        self.max.z.clone() - self.min.z.clone()
    }
}
