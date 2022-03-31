#![deny(rustdoc::missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use crate::traits::*;

mod extensions;
mod traits;
mod utils;

#[cfg(feature = "shape")]
pub use graphics_shape as shape;
