#![deny(rustdoc::missing_crate_level_docs)]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use crate::manhattan::*;

mod chebyshev;
mod euclidean;
mod hamming;
mod manhattan;
pub use crate::{
    chebyshev::ChebyshevDistance, euclidean::EuclideanDistance, hamming::HammingDistance, manhattan::ManhattanDistance,
};

pub trait FrechetDistance<T, Rhs = Self> {
    fn frechet_distance(&self, rhs: &Rhs) -> T;
}

pub trait GeodesicDistance<T, Rhs = Self> {
    // Required method
    fn geodesic_distance(&self, rhs: &Rhs) -> T;
}

pub trait HaversineDistance<T, Rhs = Self> {
    // Required method
    fn haversine_distance(&self, rhs: &Rhs) -> T;
}

pub trait MinkowskiDistance<T, Rhs = Self> {
    // Required method
    fn minkowski_distance(&self, rhs: &Rhs, p: T) -> T;
}

pub trait LevenshteinDistance<T, Rhs = Self> {
    // Required method
    fn levenshtein_distance(&self, rhs: &Rhs) -> T;
}

pub trait JaroDistance<T, Rhs = Self> {
    // Required method
    fn jaro_distance(&self, rhs: &Rhs) -> T;
}

pub trait JaroWinklerDistance<T, Rhs = Self> {
    // Required method
    fn jaro_winkler_distance(&self, rhs: &Rhs) -> T;
}

pub trait SorensenDiceDistance<T, Rhs = Self> {
    // Required method
    fn sorensen_dice_distance(&self, rhs: &Rhs) -> T;
}

pub trait CosineDistance<T, Rhs = Self> {
    // Required method
    fn cosine_distance(&self, rhs: &Rhs) -> T;
}

pub trait JaccardDistance<T, Rhs = Self> {
    // Required method
    fn jaccard_distance(&self, rhs: &Rhs) -> T;
}

pub trait OverlapDistance<T, Rhs = Self> {
    // Required method
    fn overlap_distance(&self, rhs: &Rhs) -> T;
}

pub trait RogersTanimotoDistance<T, Rhs = Self> {
    fn rogers_tanimoto_distance(&self, rhs: &Rhs) -> T;
}

pub trait RusselRaoDistance<T, Rhs = Self> {
    fn russel_rao_distance(&self, rhs: &Rhs) -> T;
}

pub trait SokalSneathDistance<T, Rhs = Self> {
    fn sokal_sneath_distance(&self, rhs: &Rhs) -> T;
}

pub trait KulczynskiDistance<T, Rhs = Self> {
    fn kulczynski_distance(&self, rhs: &Rhs) -> T;
}

pub trait TverskyDistance<T, Rhs = Self> {
    fn tversky_distance(&self, rhs: &Rhs, alpha: T, beta: T) -> T;
}

pub trait TanimotoDistance<T, Rhs = Self> {
    fn tanimoto_distance(&self, rhs: &Rhs) -> T;
}

pub trait BrayCurtisDistance<T, Rhs = Self> {
    fn bray_curtis_distance(&self, rhs: &Rhs) -> T;
}

pub trait CanberraDistance<T, Rhs = Self> {
    fn canberra_distance(&self, rhs: &Rhs) -> T;
}

pub trait ChiSquaredDistance<T, Rhs = Self> {
    fn chi_squared_distance(&self, rhs: &Rhs) -> T;
}

pub trait GowerDistance<T, Rhs = Self> {
    fn gower_distance(&self, rhs: &Rhs) -> T;
}
