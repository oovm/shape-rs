use super::*;
use crate::elements::polygon_like::Polyline;

#[allow(unused_variables)]
impl<T> Ellipse<T> {
    /// Create a new ellipse with the center and the two axes and .
    pub fn new<P>(center: P, radius: (T, T), angle: T) -> Self
    where
        P: Into<Point<T>>,
    {
        Self { center: center.into(), radius, angle }
    }
    /// Create a new ellipse with the coefficient of equation.
    ///
    /// ```math
    /// A x^2 + B y^2 + C xy + D x + E y + F = 0
    /// ```
    pub fn from_coefficient(a: T, b: T, c: T, d: T, e: T, f: T) -> Self {
        todo!()
    }

    /// Create a new ellipse with 5 points.
    pub fn from_5_points(p1: Point<T>, p2: Point<T>, p3: Point<T>, p4: Point<T>, p5: Point<T>) {}
}

impl<T> Ellipse<T>
where
    T: Zero + PartialEq,
{
    /// a / pi in Z
    pub fn is_horizontal(&self) -> bool {
        self.angle == T::zero()
    }
}

impl<T> Ellipse<T>
where
    T: Real + Pow<u32, Output = T>,
{
    /// Return the center of the ellipse.
    pub fn major_axis(&self) -> &T {
        &self.radius.0
    }
    /// Get the minor axis of the ellipse.
    pub fn minor_axis(&self) -> &T {
        &self.radius.1
    }
    /// Return the homogeneous parameters.
    /// ```math
    /// Ax^2+2Bxy+Cy^2+2Dx+2Ey+F=0
    /// ```
    pub fn homogeneous(&self) -> (T, T, T, T, T, T) {
        todo!()
    }
    /// Get the major delta of the ellipse.
    /// ```math
    /// \Delta =
    /// \begin{vmatrix}
    /// A & B & D \\
    /// B & C & E \\
    /// D & E & F \\
    /// \end{vmatrix}
    /// = ACF+2BDE-AE^2-CD^2-FB^2
    /// ```
    pub fn major_delta(&self) -> T {
        let (a, b, c, d, e, f) = self.homogeneous();
        a * c * f + two::<T>() * b * d * e - a * e * e - c * d * d - f * b * b
    }
    /// Get the minor delta of the ellipse.
    /// ```math
    /// \delta =
    /// \begin{vmatrix}
    /// A & B \\
    /// B & C \\
    /// \end{vmatrix}
    /// = AC - B^2
    /// ```
    pub fn minor_delta(&self) -> T {
        let (a, b, c, _, _, _) = self.homogeneous();
        a * c - b.pow(2)
    }
}

impl<T> Ellipse<T> {
    pub fn approx_polygon(self) -> Polygon<T> {
        todo!()
    }
    pub fn approx_polyline(self) -> Polyline<T> {
        todo!()
    }
}
