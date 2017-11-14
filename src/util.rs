/// Takes the quotient of two numbers `a/b`.
///
/// The result is rounded up, rather than the standard
/// behaviour of integer division rounding down ("truncating decimals").
///
/// Taken from
/// https://stackoverflow.com/a/2745086
pub const fn division_ceil(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

