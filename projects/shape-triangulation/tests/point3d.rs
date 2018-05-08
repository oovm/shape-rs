use projective::Projective3D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point(f64, f64, f64);

#[rustfmt::skip]
impl Projective3D<f64> for Point {
    fn transform(&self, matrix: &[&f64; 16]) -> Self {
        Point(
            matrix[0] * self.0 + matrix[1] * self.1 + matrix[2] * self.2 + matrix[3],
            matrix[4] * self.0 + matrix[5] * self.1 + matrix[6] * self.2 + matrix[7],
            matrix[8] * self.0 + matrix[9] * self.1 + matrix[10] * self.2 + matrix[11],
        )
    }
}

#[test]
fn test_transform() {
    let p0 = Point(1.0, 2.0, 3.0);
    assert_eq!(p0.translate(&3.0, &2.0, &1.0), Point(4.0, 4.0, 4.0));
    assert_eq!(p0.scale(&1.0, &2.0, &3.0), Point(1.0, 4.0, 9.0));
}
