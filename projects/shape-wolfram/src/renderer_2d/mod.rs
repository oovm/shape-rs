mod polygon_like;
use crate::SVG;
use shape_core::{Rectangle, Square};
use std::fmt::Display;

/// Mark a type that can be convert to svg
pub trait ToWolfram {
    /// Convert to svg by reference
    fn to_wolfram(&self) -> SVG;
}
