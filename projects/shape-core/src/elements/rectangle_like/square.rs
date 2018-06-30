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
        let center = Point::from(center);
        let anchor = Point::new(center.x - side.clone() / two(), center.y - side.clone() / two());
        Self { anchor, side }
    }
}
