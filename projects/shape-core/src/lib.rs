// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
pub use num_traits::{float::Float, real::Real, Num, One, Signed, Zero};

pub use crate::{elements::*, macros::*, traits::*};
pub use itertools::Itertools;

mod elements;
mod macros;
mod metrics;
mod traits;

pub use elements::*;

pub use distantia::EuclideanDistance;
