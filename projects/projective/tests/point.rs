use projective::Projective;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(f64, f64);

#[rustfmt::skip]
impl Projective<f64> for Point {
    fn transform(&mut self, matrix: &[&f64; 9]) {
        self.0 = matrix[0] * self.0 + matrix[1] * self.1+ matrix[2];
        self.1 = matrix[3] * self.0 + matrix[4] * self.1 +matrix[5];
    }
}

#[test]
fn test_transform() {
    let mut p0 = Point(1.0, 2.0);
    p0.translate(&2.0, &1.0);
    assert_eq!(p0, Point(3.0, 3.0));
    p0.scale(&2.0, &3.0);
    assert_eq!(p0, Point(6.0, 9.0));
    // floating precision error, implement rotate manually to reduce errors
    p0.rotate(&std::f64::consts::PI);
    assert_eq!(p0, Point(-5.999999999999999, -9.0));
}
