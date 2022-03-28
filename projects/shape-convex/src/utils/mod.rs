use std::ops::{Mul, Sub};

#[inline]
pub fn dot_dim2_point3<T>(a: &(T, T), b: &(T, T), c: &(T, T)) -> T
where
    T: Clone + PartialOrd,
    T: Sub<Output = T> + Mul<Output = T>,
{
    let p = (b.0.clone() - a.0.clone()) * (c.1.clone() - b.1.clone());
    let q = (b.1.clone() - a.1.clone()) * (c.0.clone() - b.0.clone());
    p - q
}
