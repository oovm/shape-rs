use crate::{Circle, Line, Point, Rectangle, Triangle};
use distantia::{EuclideanDistance, ManhattanDistance};
use itertools::Itertools;
use num_traits::{real::Real, Float, Num};
use std::{cmp::Ordering, ops::Range};

mod euclidean;
mod manhattan;
