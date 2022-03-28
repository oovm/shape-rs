use num_traits::{real::Real, One, Zero};

/// 2D projective transformations
///
/// # Examples
#[doc = "```"]
#[doc = include_str ! ("../../tests/point.rs")]
#[doc = "```"]
pub trait Projective<T>
where
    Self: Sized,
    T: Zero + One + Real,
{
    /// Transform by a 3×3 matrix.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{bmatrix}
    ///     m_0 & m_1 & m_2 \\
    ///     m_3 & m_4 & m_5 \\
    ///     m_6 & m_7 & m_8 \\
    /// \end{bmatrix}
    /// ```
    fn transform(self, matrix: &[T; 9]) -> Self;
    /// Transform by rotate degree $\alpha$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{bmatrix}
    ///     cos\alpha & sin\alpha & 0 \\
    ///    -sin\alpha & cos\alpha & 0 \\
    ///     0         & 0         & 1 \\
    /// \end{bmatrix}
    /// ```
    #[rustfmt::skip]
    fn rotate(self, angle: T) -> Self {
        self.transform(&[
            angle.cos(), angle.sin(), T::zero(),
            -angle.sin(), angle.cos(), T::zero(),
            T::zero(), T::zero(), T::one(),
        ])
    }
    /// Transform by length $\delta x, \delta y$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     1 & 0 & x \\
    ///     0 & 1 & y \\
    ///     0 & 0 & 1 \\
    /// \end{vmatrix}
    /// ```
    #[rustfmt::skip]
    fn translate(self, x: T, y: T) -> Self {
        self.transform(&[
            T::one(), T::zero(), x,
            T::zero(), T::one(), y,
            T::zero(), T::zero(), T::one(),
        ])
    }
    /// Transform by length $\delta x$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     1 & 0 & x \\
    ///     0 & 1 & 0 \\
    ///     0 & 0 & 1 \\
    /// \end{vmatrix}
    /// ```
    fn translate_x(self, x: T) -> Self {
        self.translate(x, T::zero())
    }
    /// Transform by length $\delta y$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     1 & 0 & 0 \\
    ///     0 & 1 & y \\
    ///     0 & 0 & 1 \\
    /// \end{vmatrix}
    /// ```
    fn translate_y(self, y: T) -> Self {
        self.translate(T::zero(), y)
    }
    /// Transform by scale $x, y$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     x & 0 & 0 \\
    ///     0 & y & 0 \\
    ///     0 & 0 & 1 \\
    /// \end{vmatrix}
    /// ```
    #[rustfmt::skip]
    fn scale(self, x: T, y: T) -> Self {
        self.transform(&[
            x, T::zero(), T::zero(),
            T::zero(), y, T::zero(),
            T::zero(), T::zero(), T::one(),
        ])
    }
    /// Transform by scale $x$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     x & 0 & 0 \\
    ///     0 & 1 & 0 \\
    ///     0 & 0 & 1 \\
    /// \end{vmatrix}
    /// ```
    fn scale_x(self, x: T) -> Self {
        self.scale(x, T::one())
    }
    /// Transform by scale $y$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     1 & 0 & 0 \\
    ///     0 & y & 0 \\
    ///     0 & 0 & 1 \\
    /// \end{vmatrix}
    /// ```
    fn scale_y(self, y: T) -> Self {
        self.scale(T::one(), y)
    }

    #[rustfmt::skip]
    fn reflect(self, x: T, y: T) -> Self {
        self.transform(&[
            T::one(), T::zero(), T::zero(),
            T::zero(), T::one(), T::zero(),
            x, y, T::one(),
        ])
    }
    fn reflect_x(self) -> Self {
        self.reflect(T::zero(), T::one())
    }
    fn reflect_y(self) -> Self {
        self.reflect(T::one(), T::zero())
    }
    /// Transform by angle $\alpha, \beta$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     1        & tan\alpha & 0 \\
    ///     tan\beta & 1         & 0 \\
    ///     0        & 0         & 1 \\
    /// \end{vmatrix}
    /// ```
    #[rustfmt::skip]
    fn skew(self, a: T, b: T) -> Self {
        self.transform(&[
            T::one(), a.tan(), T::zero(),
            b.tan(), T::one(), T::zero(),
            T::zero(), T::zero(), T::one(),
        ])
    }
    /// Transform by angle $\alpha$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     1        & tan\alpha & 0 \\
    ///     tan\beta & 1         & 0 \\
    ///     0        & 0         & 1 \\
    /// \end{vmatrix}
    /// ```
    fn skew_x(self, a: T) -> Self {
        self.skew(a, T::zero())
    }
    /// Transform by angle $\beta$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{vmatrix}
    ///     1        & tan\alpha & 0 \\
    ///     tan\beta & 1         & 0 \\
    ///     0        & 0         & 1 \\
    /// \end{vmatrix}
    /// ```
    fn skew_y(self, b: T) -> Self {
        self.skew(T::zero(), b)
    }
}

pub trait Projective3D<T>
where
    Self: Sized,
    T: Zero + One + Real,
{
    /// Transform by matrix.
    fn transform(self, m: &[T; 16]) -> Self;

    fn rotate(self, a: T, x: T, y: T, z: T) -> Self {
        self.transform(&[
            //
            a.cos() + x * x * (T::one() - a.cos()),
            x * y * (T::one() - a.cos()) - z * a.sin(),
            x * z * (T::one() - a.cos()) + y * a.sin(),
            T::zero(),
            //
            y * x * (T::one() - a.cos()) + z * a.sin(),
            a.cos() + y * y * (T::one() - a.cos()),
            y * z * (T::one() - a.cos()) - x * a.sin(),
            T::zero(),
            //
            z * x * (T::one() - a.cos()) - y * a.sin(),
            z * y * (T::one() - a.cos()) + x * a.sin(),
            T::zero(),
            T::zero(),
            //
            T::zero(),
            T::one(),
        ])
    }

    #[rustfmt::skip]
    fn translate(self, x: T, y: T, z: T) -> Self {
        self.transform(&[
            T::one(), T::zero(), T::zero(), x,
            T::zero(), T::one(), T::zero(), y,
            T::zero(), T::zero(), T::one(), z,
            T::zero(), T::zero(), T::zero(), T::one(),
        ])
    }
    /// Transform by length $\delta x$.
    fn translate_x(self, x: T) -> Self {
        self.translate(x, T::zero(), T::zero())
    }
    /// Transform by length $\delta y$.
    fn translate_y(self, y: T) -> Self {
        self.translate(T::zero(), y, T::zero())
    }
    fn translate_z(self, y: T) -> Self {
        self.translate(T::zero(), T::zero(), y)
    }
    #[rustfmt::skip]
    fn scale(self, x: T, y: T, z: T) -> Self {
        self.transform(&[
            x, T::zero(), T::zero(), T::zero(),
            T::zero(), y, T::zero(), T::zero(),
            T::zero(), T::zero(), z, T::zero(),
            T::zero(), T::zero(), T::zero(), T::one(),
        ])
    }
    fn scale_x(self, x: T) -> Self {
        self.scale(x, T::one(), T::one())
    }
    fn scale_y(self, y: T) -> Self {
        self.scale(T::one(), y, T::one())
    }
    fn scale_z(self, z: T) -> Self {
        self.scale(T::one(), T::one(), z)
    }
}
