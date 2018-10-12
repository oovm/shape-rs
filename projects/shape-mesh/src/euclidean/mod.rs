///
pub trait EuclideanDistance<T, Rhs = Self> {
    /// 点在物体内部时距离为0, 不要使用负距离
    fn euclidean_distance(&self, rhs: &Rhs) -> T;
    /// It is especially suitable when only the length needs to be compared
    ///
    /// a square_2d root operation is generally 14 instruction cycles
    ///
    /// and the square_2d root cannot be used in some spaces, like Z^n
    fn euclidean_squared(&self, rhs: &Rhs) -> T;
}
