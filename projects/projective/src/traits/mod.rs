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
    ///     \cos\alpha & \sin\alpha & 0 \\
    ///    -\sin\alpha & \cos\alpha & 0 \\
    ///     0          & 0          & 1 \\
    /// \end{bmatrix}
    /// ```
    #[rustfmt::skip]
    fn rotate(self, angle: T) -> Self {
        self.transform(&[
            angle.cos(), angle.sin(), T::zero(),
           -angle.sin(), angle.cos(), T::zero(),
            T::zero(),   T::zero(),   T::one(),
        ])
    }
    /// Transform by rotate with a point $\alpha$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{aligned}
    ///     x =& (x_0 - p_x)\cos α - (y_0 - p_y)\sin α + p_x \\
    ///     y =& (x_0 - p_x)\sin α + (y_0 - p_y)\cos α + p_y \\
    /// \end{aligned}
    /// ```
    fn rotate_point(self, point: &[T; 2], angle: T) -> Self {
        self.translate(-point[0], -point[1]).rotate(angle).translate(point[0], point[1])
    }
    /// Transform by length $\delta x, \delta y$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{bmatrix}
    ///     1 & 0 & x \\
    ///     0 & 1 & y \\
    ///     0 & 0 & 1 \\
    /// \end{bmatrix}
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
    /// \begin{bmatrix}
    ///     1 & 0 & x \\
    ///     0 & 1 & 0 \\
    ///     0 & 0 & 1 \\
    /// \end{bmatrix}
    /// ```
    fn translate_x(self, x: T) -> Self {
        self.translate(x, T::zero())
    }
    /// Transform by length $\delta y$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{bmatrix}
    ///     1 & 0 & 0 \\
    ///     0 & 1 & y \\
    ///     0 & 0 & 1 \\
    /// \end{bmatrix}
    /// ```
    fn translate_y(self, y: T) -> Self {
        self.translate(T::zero(), y)
    }
    /// Transform by scale $x, y$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{bmatrix}
    ///     x & 0 & 0 \\
    ///     0 & y & 0 \\
    ///     0 & 0 & 1 \\
    /// \end{bmatrix}
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
    /// \begin{bmatrix}
    ///     x & 0 & 0 \\
    ///     0 & 1 & 0 \\
    ///     0 & 0 & 1 \\
    /// \end{bmatrix}
    /// ```
    fn scale_x(self, x: T) -> Self {
        self.scale(x, T::one())
    }
    /// Transform by scale $y$.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{bmatrix}
    ///     1 & 0 & 0 \\
    ///     0 & y & 0 \\
    ///     0 & 0 & 1 \\
    /// \end{bmatrix}
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
    /// \begin{bmatrix}
    ///     1         & \tan\alpha & 0 \\
    ///     \tan\beta & 1          & 0 \\
    ///     0         & 0          & 1 \\
    /// \end{bmatrix}
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
    /// \begin{bmatrix}
    ///     1         & \tan\alpha & 0 \\
    ///     \tan\beta & 1          & 0 \\
    ///     0         & 0          & 1 \\
    /// \end{bmatrix}
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
    ///     1         & \tan\alpha & 0 \\
    ///     \tan\beta & 1          & 0 \\
    ///     0         & 0          & 1 \\
    /// \end{vmatrix}
    /// ```
    fn skew_y(self, b: T) -> Self {
        self.skew(T::zero(), b)
    }
}

/// 3D projective transformations
///
/// # Examples
#[doc = "```"]
#[doc = include_str ! ("../../tests/point3d.rs")]
#[doc = "```"]
pub trait Projective3D<T>
where
    Self: Sized,
    T: Zero + One + Real,
{
    /// Transform by a 4×4 matrix.
    ///
    /// # Matrix
    ///
    /// ```math
    /// \begin{bmatrix}
    ///     m_0  & m_1  & m_2  & m_3  \\
    ///     m_4  & m_5  & m_6  & m_7  \\
    ///     m_8  & m_9  & m_10 & m_11 \\
    ///     m_12 & m_13 & m_14 & m_15 \\
    /// \end{bmatrix}
    /// ```
    fn transform(self, m: &[T; 16]) -> Self;
    fn rotate(self, _point: &[T; 3], _alpha: T, _beta: T, _gamma: T) -> Self {
        todo!()
    }
    fn rotate_origin(self, alpha: T, beta: T, gamma: T) -> Self {
        self.rotate(&[T::zero(), T::zero(), T::zero()], alpha, beta, gamma)
    }
    /// Rotate with a line.
    #[rustfmt::skip]
    fn rotate_axis(self, _p1: &[T; 3], _p2: &[T; 3], _alpha:T) -> Self {
         todo!()
    }
    fn rotate_axis_x(self, alpha: T) -> Self {
        self.rotate_axis(&[T::zero(), T::zero(), T::zero()], &[T::one(), T::zero(), T::zero()], alpha)
    }
    fn rotate_axis_y(self, alpha: T) -> Self {
        self.rotate_axis(&[T::zero(), T::zero(), T::zero()], &[T::zero(), T::one(), T::zero()], alpha)
    }
    fn rotate_axis_z(self, alpha: T) -> Self {
        self.rotate_axis(&[T::zero(), T::zero(), T::zero()], &[T::zero(), T::zero(), T::one()], alpha)
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
