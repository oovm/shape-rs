use crate::SVG;

/// Mark a type that can be convert to svg
pub trait ToSVG3D {
    /// Convert to svg by reference
    fn to_svg(&self) -> SVG;
}
