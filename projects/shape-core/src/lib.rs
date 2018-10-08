#![allow(incomplete_features)]
#![feature(generators, iter_from_generator)]
#![feature(return_position_impl_trait_in_trait)]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use crate::{elements::*, macros::*, traits::*};
pub use distantia::EuclideanDistance;
pub use itertools::Itertools;
pub use num_traits::{float::Float, real::Real, Num, One, Signed, Zero};
mod elements;
mod macros;
mod metrics;
mod traits;
pub mod utils;

pub use elements::*;
