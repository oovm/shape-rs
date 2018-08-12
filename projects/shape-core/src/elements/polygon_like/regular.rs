use super::*;

pub struct RegularPolygon<T> {
    pub sides: usize,
    pub center: Point<T>,
    pub radius: T,
    pub rotate: T,
}

impl<T> RegularPolygon<T>
where
    T: Clone,
{
    pub fn vertexes(&self) -> Vec<Point<T>> {
        let mut out = Vec::with_capacity(self.sides);
        out
    }
    pub fn height(&self) -> T {
        self.radius.clone()
    }
    pub fn side_length(&self) -> T {
        self.radius.clone()
    }
    pub fn area(&self) -> T {
        todo!()
    }
}

impl<T> Projective<T> for RegularPolygon<T> {
    fn transform(&self, matrix: &[&T; 9]) -> Self {
        todo!()
    }
    fn rotate(&self, angle: &T) -> Self {
        self.rotate + angle
    }
}
