use super::*;

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
    /// Transform by a 3×3 matrix.
    fn rotate(self, _alpha: T, _beta: T, _gamma: T) -> Self {
        todo!()
    }
    /// Transform by a 3×3 matrix.
    fn rotate_point(self, _point: &[T; 3], _alpha: T, _beta: T, _gamma: T) -> Self {
        todo!()
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
