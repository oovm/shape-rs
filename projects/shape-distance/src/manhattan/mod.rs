///
pub trait ManhattanDistance<T, Rhs = Self> {
    /// Required method
    fn manhattan_distance(&self, rhs: &Rhs) -> T;
}
