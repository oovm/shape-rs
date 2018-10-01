pub trait HammingDistance<T, Rhs = Self> {
    // Required method
    fn hamming_distance(&self, rhs: &Rhs) -> T;
    fn hamming_weight(&self) -> T;
}
