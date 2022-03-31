use super::*;

impl<T> Parallelogram<T> {
    pub fn new<P>(anchor: P, side: (P, P)) -> Self
    where
        Point<T>: From<P>,
    {
        Self { anchor: anchor.into(), side: (side.0.into(), side.1.into()) }
    }
}

impl<T> Parallelogram<T>
where
    T: Clone + Add<Output = T>,
{
    pub fn vertex(&self) -> Vec<Point<T>> {
        let a = self.anchor.clone();
        let b = self.anchor.clone() + &self.side.0;
        let c = self.anchor.clone() + &self.side.1;
        let d = self.anchor.clone() + &self.side.0 + &self.side.1;
        vec![a, b, c, d]
    }
}
