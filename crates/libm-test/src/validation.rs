macro_rules! unary {
  ($($func:ident),*) => ($(
      paste::item! {
        #[cfg(test)]
        extern "C" { pub fn [<$func f>](x: f32) -> f32; }

        #[test]
        pub fn [<$func f _validation>]() {
            if !cfg!(optimized) { return }

            for i in 0..u32::max_value() {
                let x = f32::from_bits(i);
                let expected = unsafe { [<$func f>](x) };
                let result = libm::[<$func f>](x);
                if !crate::equalf32(expected, result) {
                  panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, expected, result);
                }
            }
        }
    }
  )*);
}

unary!(
    acos, acosh, asin, atan, cbrt, ceil, cos, cosh, erf, exp, exp2, exp10, expm1, fabs, floor, j0,
    j1, lgamma, log, log1p, log2, log10, round, sin, sinh, sqrt, tan, tanh, tgamma, trunc, y0, y1
);

#[cfg(test)]
extern "C" {
    pub fn ilogbf(x: f32) -> i32;
}

#[test]
pub fn ilogbf_validation() {
    if !cfg!(optimized) {
        return;
    }
    for i in 0..u32::max_value() {
        let x = f32::from_bits(i);
        let expected = unsafe { ilogbf(x) };
        let result = libm::ilogbf(x);
        if !crate::equali32(expected, result) {
            panic!(
                "INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}",
                x, expected, result
            );
        }
    }
}

#[cfg(test)]
extern "C" {
    pub fn modff(x: f32, y: *mut f32) -> f32;
}

#[test]
pub fn modff_validation() {
    if !cfg!(optimized) {
        return;
    }
    for i in 0..u32::max_value() {
        let mut b = 0.;
        let x = f32::from_bits(i);
        let a = unsafe { modff(x, &mut b) };
        let (c, d) = libm::modff(x);
        if !crate::equalf32(a, c) || !crate::equalf32(b, d) {
            panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
        }
    }
}

#[cfg(test)]
extern "C" {
    pub fn frexpf(x: f32, y: *mut i32) -> f32;
}

#[test]
pub fn frexpf_validation() {
    if !cfg!(optimized) {
        return;
    }
    for i in 0..u32::max_value() {
        let mut b = 0;
        let x = f32::from_bits(i);
        let a = unsafe { frexpf(x, &mut b) };
        let (c, d) = libm::frexpf(x);
        if !crate::equalf32(a, c) || !crate::equali32(b, d) {
            panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
        }
    }
}

#[cfg(test)]
extern "C" {
    pub fn sincosf(x: f32, sin: *mut f32, cos: *mut f32);
}

#[test]
pub fn sincosf_validation() {
    if !cfg!(optimized) {
        return;
    }
    for i in 0..u32::max_value() {
        let x = f32::from_bits(i);
        let mut sin = 0.;
        let mut cos = 0.;
        unsafe { sincosf(x, &mut sin, &mut cos) };
        let result = libm::sincosf(x);
        if !crate::equalf32(sin, result.0) || !crate::equalf32(cos, result.1) {
            panic!(
                "INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}",
                x,
                (sin, cos),
                result
            );
        }
    }
}
