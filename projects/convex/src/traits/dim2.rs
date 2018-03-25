use std::{
    cmp::Ordering,
    mem::take,
    ops::{Add, Mul, Sub},
};

use num_traits::{Signed, Zero};
use partition::partition;

use crate::ConvexHull;

impl<T> ConvexHull<T> for &[(T, T)] {
    type Output = Vec<(T, T)>;
    fn get_convex_hull(&self, tolerance: Option<T>) -> Option<Self::Output> {
        match self {
            []|[_]|[_,_] => None,

        }
    }
}

fn convex3<T>(a: (T, T), b: (T, T), c: (T, T), tolerance: Option<T>) -> Vec<(T, T)> {
    todo!()
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
    T: Clone + PartialOrd,
{
    todo!()
    // debug_assert!(points.len() > 0);
    // let mut points = points.iter();
    // // definitely safe
    // let (mut x, mut y) = points.next().unwrap_unchecked();
    // for point in points {
    //     if point.0 < x {
    //         x = point.0.clone();
    //     }
    //     if point.1 < y {
    //         y = point.1.clone();
    //     }
    // }
    // (x, y)
}

#[inline]
fn coord_cmp<T>(p: &(T, T), q: &(T, T)) -> Ordering
where
    T: PartialOrd,
{
    p.0.partial_cmp(&q.0).unwrap().then(p.1.partial_cmp(&q.1).unwrap())
}

/// check counter-clockwise
#[inline]
fn check_cc<T>(a: &(T, T), b: &(T, T), c: &(T, T)) -> bool
where
    T: Clone + PartialOrd + Zero,
    T: Sub<Output = T> + Mul<Output = T>,
{
    let p = (b.0.clone() - a.0.clone()) * (c.1.clone() - b.1.clone());
    let q = (b.1.clone() - a.1.clone()) * (c.0.clone() - b.0.clone());
    p - q > Zero::zero()
}

#[inline]
fn distance_power2<T>(a: &(T, T), b: &(T, T), p: &(T, T)) -> T
where
    T: Clone,
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    let orthogonal = ((a.1.clone() - b.1.clone()), (b.0.clone() - a.0.clone()));
    let p_diff = ((p.0.clone() - a.0.clone()), (p.1.clone() - a.1.clone()));
    orthogonal.0 * p_diff.0 + orthogonal.1 * p_diff.1
}

#[inline]
fn swap_remove_to_first<'a, T>(slice: &mut &'a mut [T], idx: usize) -> &'a mut T {
    let tmp = take(slice);
    tmp.swap(0, idx);
    let (h, t) = tmp.split_first_mut().unwrap();
    *slice = t;
    h
}
// Adapted from https://web.archive.org/web/20180409175413/http://www.ahristov.com/tutorial/geometry-games/convex-hull.html
pub fn convex4<T>(mut points: &mut [(T, T)]) -> Vec<(T, T)>
where
    T: Clone + PartialOrd + Signed,
{
    let mut hull = vec![];

    let (min, max) = {
        let (min_idx, mut max_idx) = minmax_index(points);
        let min = swap_remove_to_first(&mut points, min_idx);
        if max_idx == 0 {
            max_idx = min_idx;
        }
        if max_idx > 0 {
            max_idx -= 1;
        }
        let max = swap_remove_to_first(&mut points, max_idx);
        (min, max)
    };

    let (part1, _) = partition(points, |p| check_cc(max, min, p));
    hull_set(max, min, part1, &mut hull);
    hull.push(max.clone());
    let (part2, _) = partition(points, |p| check_cc(min, max, p));
    hull_set(min, max, part2, &mut hull);
    hull.push(min.clone());
    hull
}

/// Compute index of the lexicographically least and the greatest coordinate in one pass.
pub fn minmax_index<T>(pts: &[(T, T)]) -> (usize, usize)
where
    T: Signed + PartialOrd,
{
    assert_ne!(pts.len(), 0);
    let (min, max) = pts.iter().enumerate().fold((None, None), |(min, max), (idx, p)| {
        (
            if let Some((midx, min)) = min {
                if coord_cmp(p, min) == Ordering::Less { Some((idx, p)) } else { Some((midx, min)) }
            }
            else {
                Some((idx, p))
            },
            if let Some((midx, max)) = max {
                if coord_cmp(p, max) == Ordering::Greater { Some((idx, p)) } else { Some((midx, max)) }
            }
            else {
                Some((idx, p))
            },
        )
    });
    (min.unwrap().0, max.unwrap().0)
}

// recursively calculate the convex hull of a subset of points
fn hull_set<T>(a: &(T, T), b: &(T, T), mut set: &mut [(T, T)], hull: &mut Vec<(T, T)>)
where
    T: Signed + Clone + PartialOrd,
{
    match set {
        [] => return,
        [p] => {
            hull.push(p.clone());
            return;
        }
        _ => {}
    }
    let furthest_idx = set
        .iter()
        .map(|pt| distance_power2(a, b, pt))
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;
    let furthest_point = swap_remove_to_first(&mut set, furthest_idx);
    let (part1, _) = partition(set, |p| check_cc(furthest_point, b, p));
    hull_set(furthest_point, b, part1, hull);
    hull.push(furthest_point.clone());
    let (part2, _) = partition(set, |p| check_cc(a, furthest_point, p));
    hull_set(a, furthest_point, part2, hull);
}

#[test]
fn quick_hull_test_collinear() {
    let mut initial = vec![(-1., 0.), (-1., -1.), (-1., 1.), (0., 0.), (0., -1.), (0., 1.), (1., 0.), (1., -1.), (1., 1.)];
    let res = convex4(&mut initial);
    assert_eq!(res, vec![(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)]);
}
