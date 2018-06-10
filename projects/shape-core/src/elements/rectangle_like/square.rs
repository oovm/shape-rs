impl<T> Default for Square<T> {
    fn default() -> Self {
        Self { anchor: Point::default(), side: one() }
    }
}

impl<T> Square<T>
where
    T: Clone,
{
    pub fn new(center: Point<T>, side: T) -> Self {
        Self { anchor: center, side }
    }
    pub fn from_anchor(anchor: Point<T>, side: T) -> Self {
        Self { anchor: anchor + (side.clone() / T::from(2).unwrap()), side }
    }
}
