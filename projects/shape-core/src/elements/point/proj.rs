use super::*;
use crate::Distance;

impl<T> Distance<T, Self> for Point<T>
where
    T: Clone,
{
    fn distance_to(&self, other: &Self) -> T {
        let dx = self.x.clone() - other.x.clone();
        let dy = self.y.clone() - other.y.clone();
        dx * dx + dy * dy
    }
}

impl<T> Distance<T, Self> for Point3D<T>
where
    T: Clone,
{
    fn distance_to(&self, other: &Self) -> T {
        let dx = self.x.clone() - other.x.clone();
        let dy = self.y.clone() - other.y.clone();
        let dz = self.z.clone() - other.z.clone();
        dx * dx + dy * dy + dz * dz
    }
}

impl<T> Projective<T> for Point<T>
where
    T: Float,
{
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        let x = self.x.mul(*matrix[0]) + self.y.mul(*matrix[1]) + *matrix[2];
        let y = self.x.mul(*matrix[3]) + self.y.mul(*matrix[4]) + *matrix[5];
        Point { x, y }
    }
}
