use crate::{Ellipse, Line, Point, Polygon, Triangle};
use num_traits::Float;
use projective::Projective;

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
        Self { vertex: self.vertex.iter().map(|v| v.transform(matrix)).collect() }
    }
}

impl<T> Projective<T> for Ellipse<T>
where
    T: Float + Clone,
{
    #[track_caller]
    fn transform(&self, _: &[&T; 9]) -> Self {
        panic!("Can't keep the shape after projective transformation");
    }
    fn translate(&self, x: &T, y: &T) -> Self {
        Self { center: self.center + Point::new(x.clone(), y.clone()), ..self.clone() }
    }
    fn rotate(&self, angle: &T) -> Self {
        Self { angle: self.angle + angle.clone(), ..self.clone() }
    }
}
