mod polygon_like;
use crate::SVG;
use shape_core::{Rectangle, Square};
use std::fmt::Display;

pub trait ToSVG {
    fn to_svg(&self) -> SVG;
}
