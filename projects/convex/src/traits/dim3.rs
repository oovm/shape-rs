use std::cmp::Ordering;

use num_traits::Zero;
use partition::partition;

// Determines if `p_c` lies on the positive side of the
// segment `p_a` to `p_b`. In other words, whether segment
// `p_a` to `p_c` is a counter-clockwise rotation from the
// segment. We use kernels to ensure this predicate is
// exact.
#[inline]
fn is_ccw<T>(p: (T, T), q: (T, T), r: (T, T)) -> bool
where
{
    let res = (q.0 - p.0) * (r.1 - q.1) - (q.1 - p.1) * (r.0 - q.0);
    res > Zero::zero()
}
fn swap_remove_to_first<'a, T>(slice: &mut &'a mut [T], idx: usize) -> &'a mut T {
    // temporarily replace `slice` with an empty value
    let tmp = std::mem::take(slice);
    tmp.swap(0, idx);
    let (h, t) = tmp.split_first_mut().unwrap();
    *slice = t;
    h
}
// Adapted from https://web.archive.org/web/20180409175413/http://www.ahristov.com/tutorial/geometry-games/convex-hull.html
pub fn quick_hull<T>(mut points: &mut [(T, T)]) -> Vec<(T, T)>
where
{
    // can't build a hull from fewer than four points
    // if points.len() < 4 {
    //     return trivial_hull(points, false);
    // }
    let mut hull = vec![];

    let (min, max) = {
        let (min_idx, mut max_idx) = minmax_index(points);
        let min = swap_remove_to_first(&mut points, min_idx);

        // Two special cases to consider:
        // (1) max_idx = 0, and got swapped
        if max_idx == 0 {
            max_idx = min_idx;
        }

        // (2) max_idx = min_idx: then any point could be
        // chosen as max. But from case (1), it could now be
        // 0, and we should not decrement it.
        if max_idx > 0 {
            max_idx -= 1;
        }
        let max = swap_remove_to_first(&mut points, max_idx);
        (min, max)
    };

    {
        let (points, _) = partition(points, |p| is_ccw((max.0, max.1), (min.0, min.1), (p.0, p.1)));
        hull_set((max.0, max.1), (min.0, min.1), points, &mut hull);
    }
    hull.push(*max);
    let (points, _) = partition(points, |p| is_ccw((min.0, min.1), (max.0, max.1), (p.0, p.1)));
    hull_set((min.0, min.1), (max.0, max.1), points, &mut hull);
    hull.push(*min);
    hull
}

/// Compute index of the lexicographically least _and_ the
/// greatest coordinate in one pass.
///
/// Should only be called on a non-empty slice with no `nan`
/// coordinates.
pub fn minmax_index<T>(pts: &[(T, T)]) -> (usize, usize)
where
{
    assert_ne!(pts.len(), 0);
    let (min, max) = pts.iter().enumerate().fold((None, None), |(min, max), (idx, p)| {
        (
            if let Some((midx, min)) = min {
                if lex_cmp(p, min) == Ordering::Less { Some((idx, p)) } else { Some((midx, min)) }
            }
            else {
                Some((idx, p))
            },
            if let Some((midx, max)) = max {
                if lex_cmp(p, max) == Ordering::Greater { Some((idx, p)) } else { Some((midx, max)) }
            }
            else {
                Some((idx, p))
            },
        )
    });
    (min.unwrap().0, max.unwrap().0)
}

/// Compare two coordinates lexicographically: first by the
/// x coordinate, and break ties with the y coordinate.
/// Expects none of coordinates to be uncomparable (eg. nan)
#[inline]
pub fn lex_cmp<T: CoordNum>(p: &(T, T), q: &(T, T)) -> Ordering {
    p.0.partial_cmp(&q.0).unwrap().then(p.1.partial_cmp(&q.1).unwrap())
}

// recursively calculate the convex hull of a subset of points
fn hull_set<T>(a: (T, T), b: (T, T), mut set: &mut [(T, T)], hull: &mut Vec<(T, T)>) {
    if set.is_empty() {
        return;
    }
    if set.len() == 1 {
        hull.push(set[0]);
        return;
    }

    // Construct orthogonal vector to `p_b` - `p_a` We
    // compute inner product of this with `v` - `p_a` to
    // find the farthest point from the line segment a-b.
    let p_orth = ((a.1 - b.1), (b.0 - a.0));
    let furthest_idx = set
        .iter()
        .map(|pt| {
            let p_diff = ((pt.0 - a.0), (pt.1 - a.1));
            p_orth.0 * p_diff.0 + p_orth.1 * p_diff.1
        })
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0;

    // move Coordinate at furthest_point from set into hull
    let furthest_point = swap_remove_to_first(&mut set, furthest_idx);
    // points over PB
    {
        let (points, _) = partition(set, |p| is_ccw(*furthest_point, b, *p));
        hull_set(*furthest_point, b, points, hull);
    }
    hull.push(*furthest_point);
    // points over AP
    let (points, _) = partition(set, |p| is_ccw(a, *furthest_point, *p));
    hull_set(a, *furthest_point, points, hull);
}

#[test]
fn quick_hull_test_collinear() {
    let mut initial = vec![(-1., 0.), (-1., -1.), (-1., 1.), (0., 0.), (0., -1.), (0., 1.), (1., 0.), (1., -1.), (1., 1.)];
    let res = quick_hull(&mut initial);
    assert_eq!(res, vec![(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)]);
}
