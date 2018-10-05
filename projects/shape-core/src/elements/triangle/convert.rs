use super::*;
use crate::{Shape2D, ValidShape};

impl<T, P> TryFrom<(P, P, P)> for Triangle<T>
where
    Point<T>: From<P>,
    T: Num + Clone + PartialOrd,
{
    type Error = ();

    fn try_from(value: (P, P, P)) -> Result<Self, Self::Error> {
        let tri = Self { a: value.0.into(), b: value.1.into(), c: value.2.into() };
        match tri.is_valid() {
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
            [a, b, c] => Ok(Triangle { a: a.clone(), b: b.clone(), c: c.clone() }),
            _ => Err(()),
        }
    }
}

impl<T> Shape2D for Triangle<T> {
    type Value = T;

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn boundary(&self) -> Rectangle<Self::Value> {
        todo!()
    }

    fn vertices<'i, I>(&'i self) -> I
    where
        I: Iterator<Item = Point<&'i Self::Value>>,
    {
        todo!()
    }

    fn edges<'i, I>(&'i self) -> I
    where
        I: Iterator<Item = Line<&'i Self::Value>>,
    {
        todo!()
    }
}
