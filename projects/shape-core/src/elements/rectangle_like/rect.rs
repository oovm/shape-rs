use super::*;

impl<T> Rectangle<T>
where
    T: Clone + Num,
{
    pub fn new<P>(anchor: P, side: (T, T)) -> Self
    where
        P: Into<Point<T>>,
    {
        Self { anchor: anchor.into(), side }
    }
    pub fn from_center<P>(center: P, side: (T, T)) -> Self
    where
        Point<T>: From<P>,
    {
        let Point { x: x0, y: y0 } = center.into();
        let anchor = Point::new(x0 - side.0.clone() / two(), y0 - side.1.clone() / two());
        Self { anchor, side }
    }
    pub fn from_diagonal_points(p1: Point<T>, p2: Point<T>) -> Rectangle<T> {
        let size = p2.clone() - p1.clone();
        Self { anchor: p1, side: (size.x, size.y) }
    }
}

impl<T> Rectangle<T>
where
    T: Clone + Num + PartialOrd,
{
    pub fn bound_box(points: &[Point<T>]) -> Rectangle<T> {
        let mut x_min = points[0].x.clone();
        let mut x_max = points[0].x.clone();
        let mut y_min = points[0].y.clone();
        let mut y_max = points[0].y.clone();
        for p in points {
            if p.x.le(&x_min) {
                x_min = p.x.clone();
            }
            if p.x.ge(&x_max) {
                x_max = p.x.clone();
            }
            if p.y.le(&y_min) {
                y_min = p.y.clone();
            }
            if p.y.ge(&y_max) {
                y_max = p.y.clone();
            }
        }
        Rectangle::from_diagonal_points(Point::new(x_min, y_min), Point::new(x_max, y_max))
    }
}

impl<T> Rectangle<T>
where
    T: Clone + Add<Output = T>,
{
    pub fn vertexes(&self) -> Vec<Point<T>> {
        let a = self.anchor.clone();
        let b = Point { x: self.anchor.x.clone() + self.side.0.clone(), y: self.anchor.y.clone() };
        let c = Point { x: self.anchor.x.clone() + self.side.0.clone(), y: self.anchor.y.clone() + self.side.1.clone() };
        let d = Point { x: self.anchor.x.clone(), y: self.anchor.y.clone() + self.side.1.clone() };
        vec![a, b, c, d]
    }
}
