use super::*;
use crate::elements::polygon_like::Polyline;

#[allow(unused_variables)]
impl<T> Ellipse<T>
where
    T: Clone + Real,
{
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
    /// a x^2 + b y^2 + c xy + d x + e y + f = 0
    /// ```
    ///
    /// x = (2 c d-b f)/(b^2-4 a c)
    /// y = (2 a f-b d)/(b^2-4 a c)
    pub fn from_coefficient(a: T, b: T, c: T, d: T, e: T, f: T) -> Self {
        let delta_s = minor_delta(&a, &b, &c);
        let x = center_x(&a, &b, &c, &d, &e, delta_s);
        let y = center_y(&a, &b, &c, &d, &e, delta_s);

        let m = eta_invariant(&a, &b, &c);
        let n = eta_invariant(&d, &e, &f);
        let angle = (m.atan2(n) * two()).atan();
        Self { center: Point { x, y }, radius: (m, n), angle }
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
        major_delta(&a, &b, &c, &d, &e, &f)
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
        minor_delta(&a, &b, &c)
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

/// a c f + 2 b d e - a e^2 - c d^2 - f b^2
#[inline(always)]
fn major_delta<T>(a: &T, b: &T, c: &T, d: &T, e: &T, f: &T) -> T
where
    T: Clone + Real,
{
    let p1 = a.clone() + c.clone() + f.clone();
    let p2 = two::<T>() * b.clone() * d.clone() + two::<T>() * e.clone() * f.clone();
    let p3 = a.clone() * e.clone() * e.clone() + c.clone() * d.clone() * d.clone() + f.clone() * b.clone() * b.clone();
    p1 + p2 - p3
}

/// a c - b^2
#[inline(always)]
fn minor_delta<T>(a: &T, b: &T, c: &T) -> T
where
    T: Clone + Real,
{
    a.clone() * c.clone() - b.clone() * b.clone()
}

/// (a - c)^2 + 4 b^2
#[inline(always)]
fn eta_invariant<T>(a: &T, b: &T, c: &T) -> T
where
    T: Clone + Real,
{
    let p1 = a.clone() - c.clone();
    let eta = p1.powi(2) + four::<T>() * b.powi(2);
    eta.sqrt()
}

/// (b e - c d) / delta
#[inline(always)]
fn center_x<T>(_: &T, b: &T, c: &T, d: &T, e: &T, delta_s: &T) -> T
where
    T: Clone + Real,
{
    let p1 = b.clone() * e.clone() - c.clone() * d.clone();
    p1 / delta_s
}

/// (b d - a e) / delta
#[inline(always)]
fn center_y<T>(a: &T, b: &T, _: &T, d: &T, e: &T, delta_s: &T) -> T
where
    T: Clone + Real,
{
    let p1 = b.clone() * d.clone() - a.clone() * e.clone();
    p1 / delta_s
}
