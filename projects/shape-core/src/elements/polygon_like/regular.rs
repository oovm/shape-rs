use super::*;

impl<T> RegularPolygon<T>
where
    T: Real,
{
    pub fn vertexes(&self) -> Vec<Point<T>> {
        let out = Vec::with_capacity(self.sides);
        out
    }
    pub fn height(&self) -> T {
        todo!()
    }
    pub fn perimeter(&self) -> T {
        self.side_length() * T::from(self.sides).unwrap()
    }
    // central angle
    // perimeter
    pub fn side_length(&self) -> T {
        todo!()
    }
    pub fn area(&self) -> T {
        todo!()
    }
}

impl<T> Projective<T> for RegularPolygon<T>
where
    T: Real + AddAssign,
{
    #[track_caller]
    #[allow(unused_variables)]
    fn transform(&mut self, matrix: &[&T; 9]) {
        panic!("Can't keep shape after projective transform");
    }
    fn translate(&mut self, x: &T, y: &T) {
        self.center.x += *x;
        self.center.y += *y;
    }
    fn rotate(&mut self, angle: &T) {
        self.rotate += *angle
    }
}
