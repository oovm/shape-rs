use projective::Projective;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(f64, f64);

#[rustfmt::skip]
impl Projective<f64> for Point {
    fn transform(&self, matrix: &[&f64; 9]) -> Self {
        Point(
             matrix[0] * self.0 + matrix[1] * self.1+ matrix[2],
             matrix[3] * self.0 + matrix[4] * self.1 +matrix[5],
        )
    }
}

#[test]
fn test_transform() {
    let p0 = Point(1.0, 2.0);
    assert_eq!(p0.translate(&2.0, &1.0), Point(3.0, 3.0));
    assert_eq!(p0.scale(&2.0, &3.0), Point(2.0, 6.0));
    // floating precision error, implement rotate manually to reduce errors
    assert_eq!(p0.rotate(&std::f64::consts::PI), Point(-0.9999999999999998, -2.0));
}
