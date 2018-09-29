use crate::{Circle, Line, Point, Triangle};
use distantia::EuclideanDistance;
use itertools::Itertools;
use num_traits::{real::Real, Float, Num};
use std::cmp::Ordering;

// ┌────────────────────────┐
// │ From point to Geometry │
// └────────────────────────┘

impl<T> EuclideanDistance<T, Point<T>> for Point<T>
where
    T: Float,
{
    /// Minimum distance between two `Coord`s
    fn euclidean_distance(&self, c: &Point<T>) -> T {
        self.euclidean_squared(c).sqrt()
    }

    fn euclidean_squared(&self, rhs: &Point<T>) -> T {
        let dx = self.x - rhs.y;
        let dy = self.y - rhs.y;
        dx.powi(2) + dy.powi(2)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Multipoint<T> {
    pub points: Vec<Point<T>>,
}

impl<T> EuclideanDistance<T, Multipoint<T>> for Point<T>
where
    T: Float,
{
    fn euclidean_distance(&self, rhs: &Multipoint<T>) -> T {
        self.euclidean_squared(rhs).sqrt()
    }

    fn euclidean_squared(&self, rhs: &Multipoint<T>) -> T {
        rhs.points
            .iter()
            .map(|p| self.euclidean_squared(p))
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(T::zero())
    }
}

impl<T: Float> EuclideanDistance<T, Line<T>> for Point<T> {
    // distance = fabs(((y1 - y2) * x0 - (x1 - x2) * y0 + (x1 * y2 - x2 * y1)) / sqrt(pow(y1 - y2, 2) + pow(x1 - x2, 2)));
    fn euclidean_distance(&self, rhs: &Line<T>) -> T {
        let x0 = self.x;
        let y0 = self.y;
        let x1 = rhs.start.x;
        let y1 = rhs.start.y;
        let x2 = rhs.end.x;
        let y2 = rhs.end.y;
        let numerator = (y1 - y2) * x0 - (x1 - x2) * y0 + (x1 * y2 - x2 * y1);
        let denominator = (y1 - y2).powi(2) + (x1 - x2).powi(2);
        numerator.abs() / denominator.sqrt()
    }

    fn euclidean_squared(&self, rhs: &Line<T>) -> T {
        let x0 = self.x;
        let y0 = self.y;
        let x1 = rhs.start.x;
        let y1 = rhs.start.y;
        let x2 = rhs.end.x;
        let y2 = rhs.end.y;
        let numerator = (y1 - y2) * x0 - (x1 - x2) * y0 + (x1 * y2 - x2 * y1);
        let denominator = (y1 - y2).powi(2) + (x1 - x2).powi(2);
        (numerator.abs() / denominator.sqrt()).powi(2)
    }
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
