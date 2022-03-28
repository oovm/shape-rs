use super::*;

impl<T> ConvexHull<T> for Point<T> {
    type Output = Polygon<T>;

    fn is_convex_hull(&self, _: Option<T>) -> bool {
        false
    }

    fn get_convex_hull(&self, _: Option<T>) -> Option<Self::Output> {
        None
    }
}

impl<T> ConvexHull<T> for &[Point<T>]
where
    T: PartialOrd + Clone + Signed,
{
    type Output = Polygon<T>;

    fn is_convex_hull(&self, tolerance: Option<T>) -> bool {
        match self {
            [] | [_] | [_, _] => false,
            [a, b, c] => {
                let Point { x: x1, y: y1 } = a.clone();
                let Point { x: x2, y: y2 } = b.clone();
                let Point { x: x3, y: y3 } = c.clone();
                let Point { x: x4, y: y4 } = a.clone();
                ((y3 - y1) * (x2 - x1) - (x3 - x4) * (y2 - y4)).abs() <= tolerance.unwrap_or(T::zero())
            }
            _ => {
                todo!()
            }
        }
    }

    fn get_convex_hull(&self, tolerance: Option<T>) -> Option<Self::Output> {
        match self {
            [] | [_] | [_, _] => None,
            [a, b, c] => match self.is_convex_hull(tolerance) {
                true => Some(Polygon::new(vec![a, b, c])),
                false => None,
            },
            _ => {
                todo!()
            }
        }
    }

    fn merge_convex_hulls(lhs: Self::Output, other: &Self::Output) {
        todo!()
    }
}
