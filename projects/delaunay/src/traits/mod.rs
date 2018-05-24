use std::borrow::Cow;

///
pub trait Delaunay {
    ///
    type Output: Triangulation;
    /// Get the convex hull, allowing a certain numerical error
    fn delaunay(&self) -> Self::Output;
}

pub trait Voronoi {
    ///
    type Output: Triangulation;
    /// Get the convex hull, allowing a certain numerical error
    fn voronoi(&self) -> Self::Output;
}

pub trait Triangulation {
    type Triangle: ToOwned;
    type Edge: ToOwned;
    type Point: ToOwned;
    fn triangles<'a, T>(&'a self) -> T
    where
        T: Iterator<Item = Cow<'a, Self::Triangle>>;
    fn edges<'a, T>(&'a self) -> T
    where
        T: Iterator<Item = Cow<'a, Self::Edge>>;
    fn point<'a, T>(&'a self) -> T
    where
        T: Iterator<Item = Cow<'a, Self::Point>>;
}
