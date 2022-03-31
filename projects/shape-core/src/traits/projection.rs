use num_traits::Float;
use projective::Projective;

use crate::{Line, Point, Polygon, Square, Triangle};

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

impl<T> Projective<T> for Line<T>
where
    T: Float,
{
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        let start = self.start.transform(matrix);
        let end = self.end.transform(matrix);
        Line { start, end }
    }
}

impl<T> Projective<T> for Triangle<T>
where
    T: Float,
{
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        let a = self.vertex[0].transform(matrix);
        let b = self.vertex[1].transform(matrix);
        let c = self.vertex[2].transform(matrix);
        Triangle { vertex: [a, b, c] }
    }
}

impl<T> Projective<T> for Polygon<T>
where
    T: Float,
{
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        Self { vertex: self.vertex.iter().map(|p| p.transform(matrix)).collect() }
    }
}

impl<T> Projective<T> for Ellipse<T> {
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        todo!()
    }
}
