use crate::{Circle, Point};
use rand::{thread_rng, Rng};

impl<T> Circle<T>
where
    T: Clone,
{
    pub fn random_point(&self, mut rng: impl Rng) -> Point<T> {
        let x = rng.gen_range(self.x - self.radius.clone(), self.x + self.radius.clone());
        let y = rng.gen_range(self.y - self.radius.clone(), self.y + self.radius.clone());
        Point::new(x, y)
    }
}
