use super::*;

/// 2D projective transformations
///
/// ![](https://s2.ax1x.com/2019/05/29/VuEtUA.png)
///
/// ## Supported 2D operations
///
/// | Operation                          | Description |
/// |------------------------------------|-------------|
/// | [transform](Projective::transform) | ----------- |
/// | [translate](Projective::translate) | ----------- |
/// | [rotate](Projective::rotate)       | ----------- |
/// | [scale](Projective::scale)         | ----------- |
#[doc = "## Examples"]
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
    fn transform(&self, matrix: &[&T; 9]) -> Self;
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
    #[inline(always)]
    fn translate(&self, x: &T, y: &T) -> Self {
        self.transform(&[&T::one(), &T::zero(), x, &T::zero(), &T::one(), y, &T::zero(), &T::zero(), &T::one()])
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
    #[inline(always)]
    fn translate_x(&self, x: &T) -> Self {
        self.translate(x, &T::zero())
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
    #[inline(always)]
    fn translate_y(&self, y: &T) -> Self {
        self.translate(&T::zero(), y)
    }
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
    #[inline(always)]
    #[rustfmt::skip]
    fn rotate(&self, angle: &T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        self.transform(&[
            &cos,       &sin,       &T::zero(),
            &sin.neg(), &cos,       &T::zero(),
            &T::zero(), &T::zero(), &T::one(),
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
    #[inline(always)]
    fn rotate_point(&self, point: &[&T; 2], angle: &T) -> Self {
        self.translate(&point[0].neg(), &point[1].neg()).rotate(angle).translate(point[0], point[1])
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
    #[inline(always)]
    #[rustfmt::skip]
    fn scale(&self, x: &T, y: &T) -> Self {
        self.transform(&[
            x,          &T::zero(), &T::zero(),
            &T::zero(), y,          &T::zero(),
            &T::zero(), &T::zero(), &T::one(),
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
    #[inline(always)]
    fn scale_x(&self, x: &T) -> Self {
        self.scale(x, &T::one())
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
    #[inline(always)]
    fn scale_y(&self, y: &T) -> Self {
        self.scale(&T::one(), y)
    }
    /// Transform by skew $x, y$.
    #[inline(always)]
    #[rustfmt::skip]
    fn reflect(&self, x: &T, y: &T) -> Self {
        self.transform(&[
            &T::one(),  &T::zero(), &T::zero(),
            &T::zero(), &T::one(),  &T::zero(),
            x,          y,          &T::one(),
        ])
    }
    /// Transform by skew $x$.
    #[inline(always)]
    fn reflect_x(&self) -> Self {
        self.reflect(&T::zero(), &T::one())
    }
    /// Transform by skew $y$.

    #[inline(always)]
    fn reflect_y(&self) -> Self {
        self.reflect(&T::one(), &T::zero())
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
    #[inline(always)]
    #[rustfmt::skip]
    fn skew(&self, a: &T, b: &T) -> Self {
        self.transform(&[
            &T::one(),  &a.tan(),   &T::zero(),
            &b.tan(),   &T::one(),  &T::zero(),
            &T::zero(), &T::zero(), &T::one(),
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
    #[inline(always)]
    fn skew_x(&self, a: &T) -> Self {
        self.skew(a, &T::zero())
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
    #[inline(always)]
    fn skew_y(&self, b: &T) -> Self {
        self.skew(&T::zero(), b)
    }
}
