use super::*;
use crate::elements::polygons::Polyline;
use num_traits::FromPrimitive;

#[allow(unused_variables)]
impl<T> Ellipse<T>
where
    T: Clone + Real + FloatConst,
{
    /// Create a new ellipse with the center and the two axes and .
    pub fn new<P>(center: P, radius: (T, T), angle: T) -> Self
    where
        P: Into<Point<T>>,
    {
        Self { center: center.into(), radius, rotate: angle }
    }
    /// Create a new ellipse with the coefficient of equation.
    ///
    /// ```math
    /// a x^2 + b y^2 + c xy + d x + e y + f = 0
    /// ```
    pub fn from_coefficient(a: T, b: T, c: T, d: T, e: T, f: T) -> Self {
        let b = half(b);
        let d = half(d);
        let e = half(e);

        let delta_s = minor_delta(&a, &b, &c);
        let delta_m = major_delta(&a, &b, &c, &d, &e, &f);
        let eta = eta_invariant(&a, &b, &c);

        Self {
            center: Point { x: center_x(&a, &b, &c, &d, &e, &delta_s), y: center_y(&a, &b, &c, &d, &e, &delta_s) },
            radius: (major_axis(&a, &c, &delta_m, &delta_s, &eta), minor_axis(&a, &c, &delta_m, &delta_s, &eta)),
            rotate: angle(&a, &b, &c),
        }
    }

    /// Create a new ellipse with 5 points.
    pub fn from_5_points(p1: Point<T>, p2: Point<T>, p3: Point<T>, p4: Point<T>, p5: Point<T>) -> Self {
        let (a, b, c, d, e, f) = null_space(&[
            null_space_line(p1),
            null_space_line(p2),
            null_space_line(p3),
            null_space_line(p4),
            null_space_line(p5),
        ]);
        Self::from_coefficient(a, b, c, d, e, f)
    }
}

impl<T> Ellipse<T>
where
    T: Zero + PartialEq,
{
    /// a / pi in Z
    pub fn is_horizontal(&self) -> bool {
        self.rotate == T::zero()
    }
}

impl<T> Ellipse<T>
where
    T: Real,
{
    /// Return the center of the ellipse.
    pub fn major_axis(&self) -> &T {
        &self.radius.0
    }
    /// Get the minor axis of the ellipse.
    pub fn minor_axis(&self) -> &T {
        &self.radius.1
    }
    /// Return the homogeneous coefficients.
    /// ```math
    /// Ax^2 + 2Bxy + Cy^2 + 2Dx + 2Ey + F = 0
    /// ```
    #[inline]
    pub fn homogeneous(&self) -> (T, T, T, T, T, T) {
        todo!()
    }
    /// Return the coefficients.
    /// ```math
    /// a x^2 + b xy + c y^2 + d x + e y + f = 0
    /// ```
    #[inline]
    pub fn coefficients(&self) -> (T, T, T, T, T, T) {
        let (a, b, c, d, e, f) = self.homogeneous();
        (a, half(b), c, half(d), half(e), f)
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

impl<T> Ellipse<T>
where
    T: Clone + Real + FromPrimitive + FloatConst,
{
    #[track_caller]
    pub fn sample_polygon(&self, n: usize) -> Polygon<T> {
        debug_assert!(n >= 3, "at least 3 points");
        let mut vertex = Vec::with_capacity(n);
        for i in 0..n {
            let angle = T::from_usize(i).unwrap() * two_pi() / T::from_usize(n).unwrap();
            let x = self.sample_x(&angle);
            let y = self.sample_y(&angle);
            vertex.push(Point::new(x, y));
        }
        Polygon::new(vertex)
    }
    #[track_caller]
    pub fn sample_polyline(&self, n: usize) -> Polyline<T> {
        debug_assert!(n >= 3, "at least 3 points");
        let mut vertex = Vec::with_capacity(n);
        for i in 0..n {
            let angle = T::from_usize(i).unwrap() * two_pi() / T::from_usize(n).unwrap();
            let x = self.sample_x(&angle);
            let y = self.sample_y(&angle);
            vertex.push(Point::new(x, y));
        }
        todo!()
        // Polyline::new(vertex)
    }
    pub fn sample_x(&self, t: &T) -> T {
        self.radius.0 * self.rotate.cos() * t.cos() - self.radius.1 * self.rotate.sin() * t.sin() + self.center.x
    }
    pub fn sample_y(&self, t: &T) -> T {
        self.radius.0 * self.rotate.sin() * t.cos() + self.radius.1 * self.rotate.cos() * t.cos() + self.center.y
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
    p1 / delta_s.clone()
}

/// (b d - a e) / delta
#[inline(always)]
fn center_y<T>(a: &T, b: &T, _: &T, d: &T, e: &T, delta_s: &T) -> T
where
    T: Clone + Real,
{
    let p1 = b.clone() * d.clone() - a.clone() * e.clone();
    p1 / delta_s.clone()
}

#[inline(always)]
fn major_axis<T>(a: &T, c: &T, delta_m: &T, delta_s: &T, eta: &T) -> T
where
    T: Clone + Real,
{
    let p1 = (a.clone() + c.clone() + eta.clone()) * delta_s.clone();
    let axis = -double(delta_m) / p1;
    axis.sqrt()
}

#[inline(always)]
fn minor_axis<T>(a: &T, c: &T, delta_m: &T, delta_s: &T, eta: &T) -> T
where
    T: Clone + Real,
{
    let p1 = (a.clone() + c.clone() - eta.clone()) * delta_s.clone();
    let axis = -double(delta_m) / p1;
    axis.sqrt()
}

fn angle<T>(a: &T, b: &T, c: &T) -> T
where
    T: Clone + Real + FloatConst,
{
    match b.is_zero() {
        true if a < c => zero(),
        true => half(pi()),
        false if a < c => double(b).atan2(a.clone() - c.clone()),
        false => half::<T>(pi()) - double(b).atan2(a.clone() - c.clone()),
    }
}

fn null_space_line<T>(p: Point<T>) -> [T; 6]
where
    T: Clone + Real,
{
    [p.x.clone().powi(2), p.x.clone() * p.y.clone(), p.y.clone().powi(2), p.x.clone(), p.y.clone(), T::one()]
}

/// ```wolfram
/// NullSpace@{
///   {x1^2, x1 y1, y1^2, x1, y1, 1},
///   {x2^2, x2 y2, y2^2, x2, y2, 1},
///   {x3^2, x3 y3, y3^2, x3, y3, 1},
///   {x4^2, x4 y4, y4^2, x4, y4, 1},
///   {x5^2, x5 y5, y5^2, x5, y5, 1}
/// }
/// ```
fn null_space<T: Clone + Real>(_matrix: &[[T; 6]; 5]) -> (T, T, T, T, T, T) {
    todo!()
}
