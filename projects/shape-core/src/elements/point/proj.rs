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
