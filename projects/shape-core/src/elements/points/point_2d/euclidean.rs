use super::*;

impl<T> EuclideanDistance<T, Point<T>> for Point<T>
where
    T: Real,
{
    /// Minimum distance between two `Coord`s
    fn euclidean_distance(&self, c: &Point<T>) -> T {
        self.euclidean_squared(c).sqrt()
    }

    fn euclidean_squared(&self, rhs: &Point<T>) -> T {
        let dx = self.x - rhs.y;
        let dy = self.y - rhs.y;
        // x * x and x.powi(2) are the same
        // `mulsd %xmm0, %xmm0`
        dx.powi(2) + dy.powi(2)
    }
}

impl<T> EuclideanDistance<T, PointSet<T>> for Point<T>
where
    T: Float + PartialOrd,
{
    fn euclidean_distance(&self, rhs: &PointSet<T>) -> T {
        self.euclidean_squared(rhs).sqrt()
    }

    /// 适合单次查询最近的点
    fn euclidean_squared(&self, rhs: &PointSet<T>) -> T {
        match rhs.points.len() {
            0 => panic!("Multipoint must have at least one points"),
            1 => unsafe {
                let p = rhs.points.get_unchecked(0);
                self.euclidean_squared(p)
            },
            _ => unsafe {
                let xs =
                    rhs.points.iter().sorted_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal)).collect_vec();
                let (lhs, rhs) = rhs.points.split_at(xs.len() / 2);
                todo!()
            },
        }
    }
}

// It is not the minimum distance between a points and a lines
impl<T: Float> EuclideanDistance<T, Line<T>> for Point<T> {
    fn euclidean_distance(&self, rhs: &Line<T>) -> T {
        let (numerator, denominator) = point_line_ratio(self, rhs);
        numerator.abs() / denominator.sqrt()
    }

    fn euclidean_squared(&self, rhs: &Line<T>) -> T {
        let (numerator, denominator) = point_line_ratio(self, rhs);
        numerator.powi(2) / denominator
    }
}

fn point_line_ratio<T: Float>(p: &Point<T>, l: &Line<T>) -> (T, T) {
    let dy = l.s.y - l.e.y;
    let dx = l.s.x - l.e.x;
    let dt = l.s.x * l.e.y - l.e.x * l.s.y;
    let nu = dy * p.x - dx * p.y + dt;
    let de = dy.powi(2) + dx.powi(2);
    (nu, de)
}

impl<T: Float> EuclideanDistance<T, Triangle<T>> for Point<T> {
    fn euclidean_distance(&self, rhs: &Triangle<T>) -> T {
        self.euclidean_squared(rhs).sqrt()
    }

    fn euclidean_squared(&self, rhs: &Triangle<T>) -> T {
        todo!()
    }
}

impl<T: Float> EuclideanDistance<T, Circle<T>> for Point<T> {
    fn euclidean_distance(&self, rhs: &Circle<T>) -> T {
        let d = self.euclidean_distance(&rhs.center);
        let r1 = rhs.radius;
        if d > r1 { d - r1 } else { T::zero() }
    }

    fn euclidean_squared(&self, rhs: &Circle<T>) -> T {
        self.euclidean_distance(rhs).powi(2)
    }
}

#[test]
fn test() {
    let c = Circle::new(Point::new(0.0, 0.0), 1.0);
    let p = Point::new(3.0, 0.0);
    println!("{}", p.euclidean_distance(&c));
    println!("{}", p.euclidean_squared(&c));
}
