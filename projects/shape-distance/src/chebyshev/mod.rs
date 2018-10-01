pub trait ChebyshevDistance<T, Rhs = Self> {
    // Required method
    fn chebyshev_distance(&self, rhs: &Rhs) -> T;
}
