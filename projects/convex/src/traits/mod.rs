///
pub trait ConvexHull {
    ///
    type Output;
    ///
    fn is_convex_hull(&self, tolerance: Option<T>) -> bool;
    ///
    fn get_convex_hull(&self, tolerance: Option<T>) -> Option<Self::Output>;
}
