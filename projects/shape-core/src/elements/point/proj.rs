use super::*;
use crate::Distance;

impl<T> Distance<T, Self> for Point<T>
where
    T: Clone + Real,
{
    fn distance_to(&self, other: &Self) -> T {
        let dx = self.x.clone() - other.x.clone();
        let dy = self.y.clone() - other.y.clone();
        dx * dx + dy * dy
    }
}

impl<T> Distance<T, Self> for Point3D<T>
where
    T: Clone + Real,
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
    T: Real,
{
    fn transform(&mut self, matrix: &[&T; 9]) {
        self.x = self.x.mul(*matrix[0]) + self.y.mul(*matrix[1]) + *matrix[2];
        self.y = self.x.mul(*matrix[3]) + self.y.mul(*matrix[4]) + *matrix[5];
    }
}
