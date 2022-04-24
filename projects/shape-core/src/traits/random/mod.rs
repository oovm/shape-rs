use crate::{Circle, Point};
use num_traits::{real::Real, FloatConst};
use rand::{distributions::uniform::SampleUniform, Rng};

impl<T> Circle<T>
where
    T: Clone + Real + FloatConst + SampleUniform,
{
    pub fn random_point(&self, mut rng: impl Rng) -> Point<T> {
        let _ = rng.gen_range(T::zero()..(T::PI() * T::from(2.0).unwrap()));
        todo!()
    }
}
