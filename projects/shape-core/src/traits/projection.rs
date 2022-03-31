use num_traits::Float;
use projective::Projective;

use crate::{Ellipse, Line, Point, Polygon, Triangle};

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
        todo!()
    }
}

impl<T> Projective<T> for Ellipse<T>
where
    T: Float,
{
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        todo!()
    }
}
