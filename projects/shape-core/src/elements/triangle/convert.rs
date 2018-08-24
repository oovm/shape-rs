use super::*;
use crate::ValidShape;

impl<T, P> TryFrom<(P, P, P)> for Triangle<T>
where
    Point<T>: From<P>,
    T: Num + Clone + PartialOrd,
{
    type Error = ();

    fn try_from(value: (P, P, P)) -> Result<Self, Self::Error> {
        let tri = Self { vertex: [value.0.into(), value.1.into(), value.2.into()] };
        match tri.is_empty() {
            true => Ok(tri),
            false => Err(()),
        }
    }
}

impl<T> TryFrom<&Polygon<T>> for Triangle<T>
where
    T: Clone,
{
    type Error = ();

    fn try_from(value: &Polygon<T>) -> Result<Self, Self::Error> {
        match value.vertex.as_slice() {
            [a, b, c] => Ok(Triangle { vertex: [a.clone(), b.clone(), c.clone()] }),
            _ => Err(()),
        }
    }
}
