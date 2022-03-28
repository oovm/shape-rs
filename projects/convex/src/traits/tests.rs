use crate::ConvexHull;

#[test]
fn test_collinear() {
    let points = vec![(-1., 0.), (-1., -1.), (-1., 1.), (0., 0.), (0., -1.), (0., 1.), (1., 0.), (1., -1.), (1., 1.)];
    let res = points.get_convex_hull(None).unwrap();
    assert_eq!(res, vec![(1.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0)]);
}
