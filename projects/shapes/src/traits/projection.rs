use projective::Projective;

use crate::{Line, Point, Triangle};

impl<T> Projective<T> for Point<T> {
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        let x = &self.x * matrix[0] + &self.y * matrix[1] + matrix[2];
        let y = &self.x * matrix[3] + &self.y * matrix[4] + matrix[5];
        Point { x, y }
    }
}

impl<T> Projective<T> for Line<T> {
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        let start = self.start.transform(matrix);
        let end = self.end.transform(matrix);
        Line { start, end }
    }
}

impl<T> Projective<T> for Triangle<T> {
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        let a = self.vertex[0].transform(matrix);
        let b = self.vertex[1].transform(matrix);
        let c = self.vertex[2].transform(matrix);
        Triangle { vertex: [a, b, c] }
    }
}
