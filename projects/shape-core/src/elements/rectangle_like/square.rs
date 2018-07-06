use super::*;

impl<T> Square<T>
where
    T: Num + Clone,
{
    pub fn from_anchor<P>(anchor: P, side: T) -> Self
    where
        Point<T>: From<P>,
    {
        Self { anchor: Point::from(anchor), side }
    }

    pub fn from_center<P>(center: P, side: T) -> Self
    where
        Point<T>: From<P>,
    {
        let Point { x: x0, y: y0 } = center.into();
        let anchor = Point::new(x0 - side.clone() / two(), y0 - side.clone() / two());
        Self { anchor, side }
    }
}
impl<T> Square<T>
where
    T: Clone + Add<Output = T>,
{
    pub fn vertexes(&self) -> Vec<Point<T>> {
        let a = self.anchor.clone();
        let b = Point { x: self.anchor.x.clone() + self.side.clone(), y: self.anchor.y.clone() };
        let c = Point { x: self.anchor.x.clone() + self.side.clone(), y: self.anchor.y.clone() + self.side.clone() };
        let d = Point { x: self.anchor.x.clone(), y: self.anchor.y.clone() + self.side.clone() };
        vec![a, b, c, d]
    }
}
