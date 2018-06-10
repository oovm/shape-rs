use super::*;

impl<T> Default for Circle<T>
where
    T: One + Zero,
{
    fn default() -> Self {
        Self { center: Default::default(), radius: T::one() }
    }
}

impl<T> Circle<T>
where
    T: Signed + Clone,
    T: Mul,
{
    /// Create circle with the center and radius.
    pub fn new(center: Point<T>, radius: T) -> Self {
        Self { center, radius }
    }

    /// Create circle from two points on the diameter.
    pub fn from_2_points(p1: Point<T>, p2: Point<T>) -> Self {
        let two = T::one() + T::one();
        let center = Point::new((p1.x + p2.x).div(two), (p1.y + p2.y).div(two));
        Self { center, radius: p1.distance_to(&p2).div(two) }
    }

    /// Create circle that intersects the 3 points.
    pub fn from_3_points(p1: Point<T>, p2: Point<T>, p3: Point<T>) -> Self {
        let two = T::one() + T::one();

        let p12 = Point::new(p2.x - p1.x.clone(), p2.y - p1.y.clone());
        let p13 = Point::new(p3.x - p1.x.clone(), p3.y - p1.y.clone());

        let c12 = p12.x * p12.x + p12.y * p12.y;
        let c13 = p13.x * p13.x + p13.y * p13.y;
        let c123 = p12.x * p13.y - p12.y * p13.x;

        let cx = (p13.y * c12 - p12.y * c13) / c123.mul(two);
        let cy = (p12.x * c13 - p13.x * c12) / c123.mul(two);

        let center = Point::new(cx + p1.x, cy + p1.y);
        Self { center, radius: center.distance_to(&p1) }
    }
}

impl<T> Circle<T>
where
    T: Clone + Mul<Output = T> + FloatConst,
{
    /// Returns the area of the circle.
    pub fn area(&self) -> T {
        self.radius.clone() * self.radius.clone() * T::PI()
    }
    /// Returns the circumference of the circle.
    pub fn perimeter(&self) -> T {
        self.radius.clone() * two_pi()
    }
    /// Checks if the circle contains the given point.
    pub fn contains(&self, point: &Point<T>) -> bool {
        self.center.distance_to(point) <= self.radius
    }
    /// Checks if the circle intersects the given circle.
    pub fn intersects(&self, other: &Self) -> bool {
        self.center.distance_to(&other.center) <= self.radius + other.radius
    }
}
