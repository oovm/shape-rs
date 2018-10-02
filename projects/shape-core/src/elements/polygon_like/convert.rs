use super::*;

impl<T> From<&Triangle<T>> for Polygon<T>
where
    T: Clone,
{
    fn from(v: &Triangle<T>) -> Self {
        Self { vertex: vec![v.a.clone(), v.b.clone(), v.c.clone()] }
    }
}

impl<T> From<&Square<T>> for Polygon<T>
where
    T: Clone + Add<Output = T>,
{
    fn from(v: &Square<T>) -> Self {
        Self { vertex: v.vertexes() }
    }
}

impl<T> From<&Rectangle<T>> for Polygon<T>
where
    T: Clone + Add<Output = T>,
{
    fn from(v: &Rectangle<T>) -> Self {
        Self { vertex: v.vertexes() }
    }
}

impl<T> From<&Parallelogram<T>> for Polygon<T>
where
    T: Clone + Num,
{
    fn from(v: &Parallelogram<T>) -> Self {
        Self { vertex: v.vertexes() }
    }
}
