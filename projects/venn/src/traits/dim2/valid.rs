use super::*;

impl<T> ValidShape<T> for Triangle<T>
where
    T: Num + PartialOrd + Clone,
{
    fn is_empty_under_thousand(&self, thousand: T) -> bool {
        let a = Vector::from_2_points(self.vertex[0].clone(), self.vertex[1].clone());
        let b = Vector::from_2_points(self.vertex[1].clone(), self.vertex[2].clone());
        a.dx * b.dy - a.dy * b.dx < thousand
    }
}

impl<T> ValidShape<T> for Square<T>
where
    T: Zero + PartialOrd,
{
    fn is_empty_under_thousand(&self, thousand: T) -> bool {
        self.side <= thousand
    }
}

impl<T> ValidShape<T> for Rectangle<T>
where
    T: Zero + PartialOrd,
{
    fn is_empty_under_thousand(&self, thousand: T) -> bool {
        self.side.0 <= thousand && self.side.1 <= thousand
    }
}
