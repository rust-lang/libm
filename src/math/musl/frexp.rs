pub fn frexp(x: f64) -> (f64, isize) {
    let mut y = x.to_bits();
    let ee = ((y>>52) & 0x7ff) as isize;

    if ee == 0 {
        if x != 0. {
            let x1p64 = f64::from_bits(0x_43f0_0000_0000_0000);
            let (x, e) = frexp(x*x1p64);
            return (x, e - 64);
        }
        return (x, 0);
    } else if ee == 0x7ff {
        return (x, 0);
    }

    let e = ee - 0x3fe;
    y &= 0x_800f_ffff_ffff_ffff;
    y |= 0x_3fe0_0000_0000_0000;
    (f64::from_bits(y), e)
}
