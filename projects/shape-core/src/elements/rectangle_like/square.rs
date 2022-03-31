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
