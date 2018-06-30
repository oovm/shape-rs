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
    T: Clone + Num,
{
    pub fn vertex(&self) -> Vec<Point<T>> {
        let a = self.anchor.clone();
        let b = self.anchor.clone() + &self.side.0;
        let c = self.anchor.clone() + &self.side.1;
        let d = self.anchor.clone() + &self.side.0 + &self.side.1;
        vec![a, b, c, d]
    }
    pub fn vectors(&self) -> (Line<T>, Line<T>) {
        let a = Line::new(&self.anchor, &self.side.0);
        let b = Line::new(&self.anchor, &self.side.1);
        (a, b)
    }
    pub fn is_square(&self) -> bool
    where
        T: Float,
    {
        self.is_rectangle() && self.is_diamond()
    }
    pub fn is_diamond(&self) -> bool
    where
        T: Float,
    {
        let (a, b) = self.vectors();
        a.length() == b.length()
    }
    pub fn is_rectangle(&self) -> bool {
        let a = Line::new(&self.anchor, &self.side.0);
        let b = Line::new(&self.anchor, &self.side.1);
        a.is_orthogonal(&b)
    }
}
