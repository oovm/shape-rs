use crate::ConvexHull;

impl<T> ConvexHull<T> for &mut [(T, T)] {
    type Output = Vec<(T, T)>;
    fn get_convex_hull(&self, tolerance: Option<T>) -> Option<Self::Output> {
        let mut points = self.clone();
        match points {
            [] | [_] | [_, _] => None,
            [mut a, mut b, mut c] => match convex_hull3(a, b, c, tolerance) {
                true => Some(vec![a.clone(), b, c]),
                false => None,
            },
            _ => {
                todo!()
            }
        }
    }
}

fn convex_hull3(a: (T, T), b: (T, T), c: (T, T), tolerance: Option<T>) -> Vec<(T, T)> {
    let Point { x: x1, y: y1 } = a.clone();
    let Point { x: x2, y: y2 } = b.clone();
    let Point { x: x3, y: y3 } = c.clone();
    let Point { x: x4, y: y4 } = a.clone();
    let delta = (y3 - y1) * (x2 - x1) - (x3 - x4) * (y2 - y4);
    match delta.abs() <= tolerance.unwrap_or(T::zero()) {
        true => Some(vec![a, b, c]),
        false => None,
    }
}

unsafe fn find_min_point<T>(points: &[(T, T)]) -> (T, T)
where
    T: Clone,
{
    debug_assert!(points.len() > 0);
    let mut points = points.iter();
    // definitely safe
    let (mut x, mut y) = points.next().unwrap_unchecked();
    for point in points {
        if point.0 < x {
            x = point.0.clone();
        }
        if point.1 < y {
            y = point.1.clone();
        }
    }
    (x, y)
}
