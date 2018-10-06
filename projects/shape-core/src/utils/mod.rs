#[inline(always)]
pub fn min2<'a, 'b, 'c, T: PartialOrd>(a: &'a T, b: &'b T) -> &'c T
where
    'a: 'c,
    'b: 'c,
{
    if a < b { a } else { b }
}
#[inline(always)]
pub fn max2<'a, 'b, 'c, T: PartialOrd>(a: &'a T, b: &'b T) -> &'c T
where
    'a: 'c,
    'b: 'c,
{
    if a > b { a } else { b }
}
#[inline(always)]
pub fn min3<'a, 'b, 'c, 'd, T: PartialOrd>(a: &'a T, b: &'b T, c: &'c T) -> &'d T
where
    'a: 'd,
    'b: 'd,
    'c: 'd,
{
    min2(min2(a, b), c)
}
#[inline(always)]
pub fn max3<'a, 'b, 'c, 'd, T: PartialOrd>(a: &'a T, b: &'b T, c: &'c T) -> &'d T
where
    'a: 'd,
    'b: 'd,
    'c: 'd,
{
    max2(max2(a, b), c)
}
