///
pub trait ConvexHull {
    ///
    type Output;
    ///
    fn is_convex_hull(&self) -> bool;
    ///
    fn get_convex_hull(&self) -> Option<Self::Output>;
}
