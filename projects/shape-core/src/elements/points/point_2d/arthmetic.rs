use super::*;

impl<T> Projective<T> for Point<T>
where
    T: Real,
{
    fn transform(&mut self, matrix: &[&T; 9]) {
        self.x = self.x.mul(*matrix[0]) + self.y.mul(*matrix[1]) + *matrix[2];
        self.y = self.x.mul(*matrix[3]) + self.y.mul(*matrix[4]) + *matrix[5];
    }
}

impl<T, V> Add<V> for Point<T>
where
    Point<T>: From<V>,
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        let rhs: Self = rhs.into();
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T, V> Sub<V> for Point<T>
where
    Point<T>: From<V>,
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs: Self = rhs.into();
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
