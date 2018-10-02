use std::{
    cmp::Ordering,
    mem::take,
    ops::{Add, Mul, Sub},
};

use num_traits::Signed;
use partition::partition;

use graphics_shape::{Point, Polygon};

use crate::ConvexHull;

impl<T> ConvexHull<T> for Vec<Point<T>>
where
    T: Signed + Clone + PartialOrd,
{
    type Output = Vec<Point<T>>;
    fn get_convex_hull(&self, tolerance: Option<T>) -> Option<Self::Output> {
        match self.as_slice() {
            [] | [_] | [_, _] => None,
            [a, b, c] => convex3(a, b, c, tolerance),
            _ => Some(convex4(&mut self.clone(), tolerance)),
        }
    }
}

#[inline]
fn coord_cmp<T>(p: &Point<T>, q: &Point<T>) -> Ordering
where
    T: PartialOrd,
{
    p.x.partial_cmp(&q.x).unwrap().then(p.y.partial_cmp(&q.y).unwrap())
}

#[inline]
fn distance_power2<T>(a: &Point<T>, b: &Point<T>, p: &Point<T>) -> T
where
    T: Clone,
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    let orthogonal = ((a.y.clone() - b.y.clone()), (b.x.clone() - a.x.clone()));
    let p_diff = ((p.x.clone() - a.x.clone()), (p.y.clone() - a.y.clone()));
    orthogonal.x * p_diff.x + orthogonal.y * p_diff.y
}

#[inline]
fn swap_remove_to_first<'a, T>(slice: &mut &'a mut [T], idx: usize) -> &'a mut T {
    let tmp = take(slice);
    tmp.swap(0, idx);
    let (h, t) = tmp.split_first_mut().unwrap();
    *slice = t;
    h
}

fn convex3<T>(a: &Point<T>, b: &Point<T>, c: &Point<T>, tolerance: Option<T>) -> Option<Polygon<T>>
where
    T: Signed + Clone + PartialOrd,
{
    match a.cross_dot(b, c).abs() <= tolerance.unwrap_or(T::zero()) {
        true => Some(Polygon::new(vec![a, b, c])),
        false => None,
    }
}

// Adapted from https://web.archive.org/web/20180409175413/http://www.ahristov.com/tutorial/geometry-games/convex-hull.html
pub fn convex4<T>(mut points: &mut [Point<T>], _tolerance: Option<T>) -> Polygon<T>
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

    let (part1, _) = partition(points, |p| max.cross_dot(min, p) > T::zero());
    hull_set(max, min, part1, &mut hull);
    hull.push(max.clone());
    let (part2, _) = partition(points, |p| min.cross_dot(max, p) > T::zero());
    hull_set(min, max, part2, &mut hull);
    hull.push(min.clone());
    hull
}

/// Compute index of the lexicographically least and the greatest coordinate in one pass.
pub fn minmax_index<T>(pts: &[Point<T>]) -> (usize, usize)
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
    (min.unwrap().x, max.unwrap().x)
}

// recursively calculate the shape-mesh hull of a subset of points
fn hull_set<T>(a: &Point<T>, b: &Point<T>, mut set: &mut [Point<T>], hull: &mut Vec<Point<T>>)
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
        .x;
    let furthest_point = swap_remove_to_first(&mut set, furthest_idx);
    let (part1, _) = partition(set, |p| furthest_point.cross_dot(b, p) > T::zero());
    hull_set(furthest_point, b, part1, hull);
    hull.push(furthest_point.clone());
    let (part2, _) = partition(set, |p| a.cross_dot(furthest_point, p) > T::zero());
    hull_set(a, furthest_point, part2, hull);
}
