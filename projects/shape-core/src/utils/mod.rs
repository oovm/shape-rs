#![doc = include_str!("readme.md")]

/// Returns the minimum of two values.
#[inline(always)]
pub fn min2<'a, 'b, 'c, T>(a: &'a T, b: &'b T) -> &'c T
where
    T: PartialOrd,
    'a: 'c,
    'b: 'c,
{
    if a < b { a } else { b }
}
/// Returns the maximum of two values.
#[inline(always)]
pub fn max2<'a, 'b, 'c, T>(a: &'a T, b: &'b T) -> &'c T
where
    T: PartialOrd,
    'a: 'c,
    'b: 'c,
{
    if a > b { a } else { b }
}
/// Returns the minimum of three values.
#[inline(always)]
pub fn min3<'a, 'b, 'c, 'd, T>(a: &'a T, b: &'b T, c: &'c T) -> &'d T
where
    T: PartialOrd,
    'a: 'd,
    'b: 'd,
    'c: 'd,
{
    min2(min2(a, b), c)
}
/// Returns the maximum of three values.
#[inline(always)]
pub fn max3<'a, 'b, 'c, 'd, T>(a: &'a T, b: &'b T, c: &'c T) -> &'d T
where
    T: PartialOrd,
    'a: 'd,
    'b: 'd,
    'c: 'd,
{
    max2(max2(a, b), c)
}
