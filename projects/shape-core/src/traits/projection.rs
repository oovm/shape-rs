use crate::{Ellipse, Line, Polygon, Triangle};
use num_traits::Float;
use projective::Projective;
use std::ops::AddAssign;

impl<T> Projective<T> for Line<T>
where
    T: Float,
{
    fn transform(&mut self, matrix: &[&T; 9]) {
        self.start.transform(matrix);
        self.end.transform(matrix);
    }
}

impl<T> Projective<T> for Triangle<T>
where
    T: Float,
{
    fn transform(&mut self, matrix: &[&T; 9]) {
        self.vertex[0].transform(matrix);
        self.vertex[1].transform(matrix);
        self.vertex[2].transform(matrix);
    }
}

impl<T> Projective<T> for Polygon<T>
where
    T: Float,
{
    fn transform(&mut self, matrix: &[&T; 9]) {
        for point in self.vertex.iter_mut() {
            point.transform(matrix);
        }
    }
}

impl<T> Projective<T> for Ellipse<T>
where
    T: Float + AddAssign,
{
    #[track_caller]
    fn transform(&mut self, _: &[&T; 9]) {
        panic!("Can't keep the shape after projective transformation");
    }
    fn translate(&mut self, x: &T, y: &T) {
        self.center.x += *x;
        self.center.y += *y;
    }
    fn rotate(&mut self, angle: &T) {
        self.rotate += *angle;
    }
}
