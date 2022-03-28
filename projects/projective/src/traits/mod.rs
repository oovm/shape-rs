use num_traits::{real::Real, One, Zero};


///
///
/// # Arguments
///
/// * `matrix`:
///
/// returns: Self
///
/// # Examples
///
/// ```
///
/// ```
pub trait Projective<T>
where
    Self: Sized,
    T: Zero + One + Real,
{
    /// Transform by a 3×3 matrix.
    ///
    /// # Arguments
    ///
    /// * `matrix`: $$$$
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn transform(self, matrix: &[T; 9]) -> Self;
    #[rustfmt::skip]
    fn rotate(self, angle: T) -> Self {
        self.transform(&[
             angle.cos(), angle.sin(), T::zero(),
            -angle.sin(), angle.cos(), T::zero(),
             T::zero(),   T::zero(),   T::one(),
        ])
    }
    /// Translate by x, y
    #[rustfmt::skip]
    fn translate(self, x: T, y: T) -> Self {
        self.transform(&[
            T::one(),  T::zero(), x,
            T::zero(), T::one(),  y,
            T::zero(), T::zero(), T::one(),
        ])
    }
    /// Scale by x, y
    fn translate_x(self, x: T) -> Self {
        self.translate(x, T::zero())
    }
    fn translate_y(self, y: T) -> Self {
        self.translate(T::zero(), y)
    }
    #[rustfmt::skip]
    fn scale(self, x: T, y: T) -> Self {
        self.transform(&[
            x,         T::zero(), T::zero(),
            T::zero(), y,         T::zero(),
            T::zero(), T::zero(), T::one(),
        ])
    }
    fn scale_x(self, x: T) -> Self {
        self.scale(x, T::one())
    }
    fn scale_y(self, y: T) -> Self {
        self.scale(T::one(), y)
    }
    #[rustfmt::skip]
    fn reflect(self, x: T, y: T) -> Self {
        self.transform(&[
            T::one(),  T::zero(), T::zero(),
            T::zero(), T::one(),  T::zero(),
            x,         y,         T::one(),
        ])
    }
    fn reflect_x(self) -> Self {
        self.reflect(T::zero(), T::one())
    }
    fn reflect_y(self) -> Self {
        self.reflect(T::one(), T::zero())
    }
    #[rustfmt::skip]
    fn skew(self, x: T, y: T) -> Self {
        self.transform(&[
            T::one(),  x,         T::zero(),
            y,         T::one(),  T::zero(),
            T::zero(), T::zero(), T::one(),
        ])
    }
    fn skew_x(self, x: T) -> Self {
        self.skew(x, T::zero())
    }
    fn skew_y(self, y: T) -> Self {
        self.skew(T::zero(), y)
    }
}
