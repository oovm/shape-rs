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
        // Handling special cases
        match self {
            [] | [_] | [_, _] => return false,
            [a, b, c] => {}
            _ => {}
        }
        // Use divide and conquer algorithms for large-scale problems
        const PARALLEL_IS_FASTER: usize = 131072;
        match self.len() >= PARALLEL_IS_FASTER {
            true => {}
            false => {}
        }
        todo!()
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
