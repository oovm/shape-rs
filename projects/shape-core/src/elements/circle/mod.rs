use crate::{π, Circle, Float, Point};

impl Circle {
    /// Create a new circle with the given center and radius.
    pub fn unit(x: Float, y: Float) -> Self {
        Self { center: Point::new(x, y), radius: 1.0 }
    }
    /// Create circle with the center and radius.
    pub fn from_1_point(center: &Point, radius: Float) -> Self {
        Self { center: Point { x: center.x, y: center.y }, radius }
    }

    /// Create circle from two points on the diameter.
    pub fn from_2_points(p1: &Point, p2: &Point) -> Self {
        let center = Point::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0);
        Self { center, radius: p1.distance_to(p2) / 2.0 }
    }

    /// Create circle that intersects the 3 points.
    pub fn from_3_points(p1: &Point, p2: &Point, p3: &Point) -> Self {
        let p12 = Point::new(p2.x - p1.x, p2.y - p1.y);
        let p13 = Point::new(p3.x - p1.x, p3.y - p1.y);

        let c12 = p12.x * p12.x + p12.y * p12.y;
        let c13 = p13.x * p13.x + p13.y * p13.y;
        let c123 = p12.x * p13.y - p12.y * p13.x;

        let cx = (p13.y * c12 - p12.y * c13) / (2.0 * c123);
        let cy = (p12.x * c13 - p13.x * c12) / (2.0 * c123);

        let center = Point::new(cx + p1.x, cy + p1.y);
        Self { center, radius: center.distance_to(p1) }
    }
    /// TeX representation of the circle.
    pub fn tex_normal(&self) -> String {
        let x0 = self.center.x;
        let y0 = self.center.y;
        let r = self.radius;
        format!("(x-{x0})^2+(y-{y0})^2 = {r}^2")
    }
}

impl Circle {
    /// Returns the area of the circle.
    pub fn area(&self) -> Float {
        π * self.radius * self.radius
    }
    /// Returns the circumference of the circle.
    pub fn perimeter(&self) -> Float {
        2.0 * π * self.radius
    }
    /// Checks if the circle contains the given point.
    pub fn contains(&self, point: &Point) -> bool {
        self.center.distance_to(point) <= self.radius
    }
    /// Checks if the circle intersects the given circle.
    pub fn intersects(&self, other: &Self) -> bool {
        self.center.distance_to(&other.center) <= self.radius + other.radius
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self { center: Point::default(), radius: 0.0 }
    }
}
