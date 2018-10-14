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
    pub fn vertexes(&self) -> Vec<Point<T>> {
        let a = self.anchor.clone();
        let b = self.anchor.clone() + &self.side.0;
        let c = self.anchor.clone() + &self.side.1;
        let d = self.anchor.clone() + &self.side.0 + &self.side.1;
        vec![a, b, c, d]
    }
    pub fn side_edges(&self) -> (Line<T>, Line<T>) {
        let a = Line::new(&self.anchor, &self.side.0);
        let b = Line::new(&self.anchor, &self.side.1);
        (a, b)
    }
    /// Check if the parallelogram is a square_2d
    pub fn is_square(&self) -> bool
    where
        T: Float,
    {
        self.is_rectangle() && self.is_diamond()
    }
    /// Check if the parallelogram is a diamond
    pub fn is_diamond(&self) -> bool
    where
        T: Float,
    {
        let (a, b) = self.side_edges();
        a.length() == b.length()
    }
    /// Check if the parallelogram is a rectangle_2d
    pub fn is_rectangle(&self) -> bool {
        let a = Line::new(&self.anchor, &self.side.0);
        let b = Line::new(&self.anchor, &self.side.1);
        a.is_orthogonal(&b)
    }
}
