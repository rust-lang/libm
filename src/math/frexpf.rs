/// Split floating-point number (f32)
///
/// All nonzero, normal numbers can be described as `m*2^p`.
/// Represents the double *val* as a mantissa *m* and a power of two *p*.
/// The resulting mantissa will always be greater than or equal to `0.5`,
/// and less than `1.0` (as long as *val* is nonzero). The power of two will be stored in *exp*.
/// *m* and *p* are calculated so that *val* is *m* times `2` to the power *p*.
pub fn frexpf(x: f32) -> (f32, i32) {
    let mut y = x.to_bits();
    let ee: i32 = ((y >> 23) & 0xff) as i32;

    if ee == 0 {
        if x != 0. {
            let x1p64 = f32::from_bits(0x_5f80_0000);
            let (x, e) = frexpf(x * x1p64);
            return (x, e - 64);
        } else {
            return (x, 0);
        }
    } else if ee == 0xff {
        return (x, 0);
    }

    let e = ee - 0x7e;
    y &= 0x_807f_ffff;
    y |= 0x_3f00_0000;
    (f32::from_bits(y), e)
}
