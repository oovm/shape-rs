use std::ops::{Mul, Sub};

use graphics_shape::Zero;

use super::*;

impl<T> ConvexHull for Point<T> {
    type Output = ();

    fn is_convex_hull(&self) -> bool {
        false
    }

    fn get_convex_hull(&self) -> Option<Self::Output> {
        None
    }
}

impl<T> ConvexHull for &[Point<T>]
where
    T: Zero + PartialOrd,
    T: Sub<Output = T> + Mul<Output = T>,
{
    type Output = ();

    fn is_convex_hull(&self) -> bool {
        match self {
            [] | [_] | [_, _] => false,
            [a, b, c] => (a.x - c.x) * (b.y - c.y) == (a.y - c.y) * (b.x - c.x),
            _ => {
                todo!()
                // let mut is_convex = true;
                // for i in 0..self.len() {
                //     let a = self[i];
                //     let b = self[(i + 1) % self.len()];
                //     let c = self[(i + 2) % self.len()];
                //     if (a.x - c.x) * (b.y - c.y) - (a.y - c.y) * (b.x - c.x) < T::zero() {
                //         is_convex = false;
                //         break;
                //     }
                // }
                // is_convex
            }
        }
    }

    fn get_convex_hull(&self) -> Option<Self::Output> {
        todo!()
    }
}
