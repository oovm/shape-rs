use super::*;

impl<T> ValidShape<T> for Triangle<T>
where
    T: Zero,
{
    fn is_empty_under_thousand(&self, thousand: T) -> bool {
        todo!()
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
