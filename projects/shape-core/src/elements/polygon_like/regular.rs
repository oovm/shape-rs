use super::*;

impl<T> RegularPolygon<T>
where
    T: Real,
{
    pub fn unit(sides: usize) -> Self {
        Self { sides, center: Point { x: T::zero(), y: T::zero() }, radius: T::one(), rotate: T::zero() }
    }
    pub fn new<P>(sides: usize, center: P, radius: T, rotate: T) -> Self
    where
        Point<T>: From<P>,
    {
        Self { sides, center: center.into(), radius, rotate }
    }

    pub fn vertexes(&self) -> Vec<Point<T>> {
        let mut out = Vec::with_capacity(self.sides);
        for i in 0..self.sides {
            let mut vertex = self.original_vertexes(i);
            vertex.rotate(&self.rotate);
            vertex.translate(&self.center.x, &self.center.y);
            out.push(vertex)
        }
        out
    }
    fn original_vertexes(&self, _index: usize) -> Point<T> {
        todo!()
    }
    pub fn height(&self) -> T {
        todo!()
    }
    pub fn perimeter(&self) -> T {
        self.side_length() * T::from(self.sides).unwrap()
    }
    // central angle
    pub fn central_angle(&self) -> T {
        todo!()
    }
    // perimeter
    pub fn side_length(&self) -> T {
        todo!()
    }
    // n r^2 tan t / 2
    pub fn area(&self) -> T {
        T::from(self.sides).unwrap() * self.radius.powi(2) * self.central_angle().tan() / two()
    }
}

impl<T> CirclePoints<T>
where
    T: Real,
{
    #[inline]
    pub fn unit(sides: usize) -> Vec<Point<T>> {
        RegularPolygon::unit(sides).vertexes()
    }
    #[inline]
    pub fn new<P>(sides: usize, center: P, radius: T, rotate: T) -> Vec<Point<T>>
    where
        Point<T>: From<P>,
    {
        RegularPolygon::new(sides, center, radius, rotate).vertexes()
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
