#[cfg(test)]
use core::{f32, f64};
#[cfg(test)]
use rand::Rng;

macro_rules! forward {
  ($func:ident, $x:ident) => (
      paste::item! {
        #[cfg(test)]
        extern "C" { pub fn [<$func>](x: $x) -> $x; }

        #[test]
        pub fn [<$func _matches>]() {
            let mut r = rand::thread_rng();
            for x in [$x::NAN, -$x::NAN, $x::INFINITY, $x::NEG_INFINITY].iter() {
                let expected = unsafe { $func(*x) };
                let result = libm::$func(*x);
                if  !crate::[<equal $x>](expected, result) {
                    panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, expected, result);
                }
            }
            for _ in 0..500 {
                let x = r.gen::<$x>();
                let expected = unsafe { $func(x) };
                let result = libm::$func(x);
                if  !crate::[<equal $x>](expected, result) {
                    panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, expected, result);
                }
            }
        }
    }
  );
  ($func:ident, $x:ty, $y:ty) => (
    paste::item! {
      #[cfg(test)]
      extern "C" { pub fn [<$func>](x: $x, y: $y) -> $x; }

      #[test]
      pub fn [<$func _matches>]() {
          let mut r = rand::thread_rng();
          for _ in 0..500 {
              let x = r.gen::<$x>();
              let y = r.gen::<$y>();
              let expected = unsafe { $func(x, y) };
              let result = libm::$func(x, y);
              if  !crate::[<equal $x>](expected, result) {
                  panic!("INPUT: {:?} {:?} EXPECTED: {:?} ACTUAL {:?}", x, y, expected, result);
              }
          }
      }
   }
);
  ($func:ident, $x:ty, $y:ty, $z:ty) => (
      paste::item! {
        #[cfg(test)]
        extern "C" { pub fn [<$func>](x: $x, y: $y, z: $z) -> $x; }

        #[test]
        pub fn [<$func _matches>]() {
            let mut r = rand::thread_rng();
            for _ in 0..500 {
              let x = r.gen::<$x>();
              let y = r.gen::<$y>();
              let z = r.gen::<$z>();
              let expected = unsafe { $func(x, y, z) };
              let result = libm::$func(x, y, z);
              if  !crate::[<equal $x>](expected, result) {
                  panic!("INPUT: {:?} {:?} {:?} EXPECTED: {:?} ACTUAL {:?}", x, y, z, expected, result);
              }
            }
        }
     }
  );
}
macro_rules! bessel {
  ($($func:ident),*) => ($(
      paste::item! {
        #[cfg(test)]
        extern "C" { pub fn [<$func>](n: i32, x: f64) -> f64; }

        #[test]
        pub fn [<$func _matches>]() {
            let mut r = rand::thread_rng();
            for _ in 0..500 {
              let mut n = r.gen::<i32>();
              n &= 0xffff;
              let x = r.gen::<f64>();
              let expected = unsafe { [<$func>](n, x) };
              let result = libm::[<$func>](n, x);
              if  !crate::equalf64(expected, result) {
                  panic!("INPUT: {:?} {:?} EXPECTED: {:?} ACTUAL {:?}", n, x, expected, result);
              }
            }
        }

      #[cfg(test)]
      extern "C" { pub fn [<$func f>](n: i32, x: f32) -> f32; }

      #[test]
      pub fn [<$func f _matches>]() {
          let mut r = rand::thread_rng();
          for _ in 0..500 {
            let mut n = r.gen::<i32>();
            n &= 0xffff;
            let x = r.gen::<f32>();
            let expected = unsafe { [<$func f>](n, x) };
            let result = libm::[<$func f>](n, x);
            if  !crate::equalf32(expected, result) {
                panic!("INPUT: {:?} {:?} EXPECTED: {:?} ACTUAL {:?}", n, x, expected, result);
            }
          }
      }
    }
)*);
}

macro_rules! unary {
  ($($func:ident),*) => ($(
    paste::item! {
      forward!($func, f64);
      forward!([<$func f>], f32);
    }
  )*);
}

macro_rules! binary {
  ($($func:ident),*) => ($(
    paste::item! {
      forward!($func, f64, f64);
      forward!([<$func f>], f32, f32);
    }
  )*);
}

macro_rules! trinary {($($func:ident),*) => {$(
      paste::item! {
        forward!($func, f64, f64, f64);
        forward!([<$func f>], f32, f32, f32);
      }
)*}}

unary!(
    acos, acosh, asin, atan, cbrt, ceil, cos, cosh, erf, exp, exp2, exp10, expm1, fabs, floor, j0,
    j1, lgamma, log, log1p, log2, log10, round, sin, sinh, sqrt, tan, tanh, tgamma, trunc, y0, y1
);
binary!(atan2, copysign, fdim, fmax, fmin, fmod, hypot, pow);
trinary!(fma);
bessel!(jn, yn);

// special cases
paste::item! {
    #[cfg(test)]
    extern "C" { pub fn scalbn(x: f64, n: i32) -> f64; }

    #[test]
    pub fn scalbn_matches() {
        let mut r = rand::thread_rng();
        for _ in 0..500 {
          let n = r.gen::<i32>();
          let x = r.gen::<f64>();
          let expected = unsafe { scalbn(x, n) };
          let result = libm::scalbn(x, n);
          if  !crate::equalf64(expected, result) {
              panic!("INPUT: {:?} {:?} EXPECTED: {:?} ACTUAL {:?}", x, n, expected, result);
          }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn scalbnf(x: f32, n: i32) -> f32; }

    #[test]
    pub fn scalbnf_matches() {
        let mut r = rand::thread_rng();
        for _ in 0..500 {
          let n = r.gen::<i32>();
          let x = r.gen::<f32>();
          let expected = unsafe { scalbnf(x, n) };
          let result = libm::scalbnf(x, n);
          if  !crate::equalf32(expected, result) {
              panic!("INPUT: {:?} {:?} EXPECTED: {:?} ACTUAL {:?}", x, n, expected, result);
          }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn ilogb(x: f64) -> i32; }

    #[test]
    pub fn ilogb_matches() {
      for x in [f64::NAN, -f64::NAN, f64::INFINITY, f64::NEG_INFINITY].iter() {
          let expected = unsafe { ilogb(*x) };
          let result = libm::ilogb(*x);
          if  !crate::equali32(expected, result) {
              panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, expected, result);
          }
      }

      let mut r = rand::thread_rng();
      for _ in 0..500 {
        let x = r.gen::<f64>();
        let expected = unsafe { ilogb(x) };
        let result = libm::ilogb(x);
        if  !crate::equali32(expected, result) {
            panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, expected, result);
        }
      }
    }

    #[cfg(test)]
    extern "C" { pub fn ilogbf(x: f32) -> i32; }

    #[test]
    pub fn ilogbf_matches() {
        for x in [f32::NAN, -f32::NAN, f32::INFINITY, f32::NEG_INFINITY].iter() {
            let expected = unsafe { ilogbf(*x) };
            let result = libm::ilogbf(*x);
            if  !crate::equali32(expected, result) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, expected, result);
            }
        }

        let mut r = rand::thread_rng();
        for _ in 0..500 {
          let x = r.gen::<f32>();
          let expected = unsafe { ilogbf(x) };
          let result = libm::ilogbf(x);
          if  !crate::equali32(expected, result) {
              panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, expected, result);
          }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn modf(x: f64, y: *mut f64) -> f64; }

    #[test]
    pub fn modf_matches() {
        for x in [f64::NAN, -f64::NAN, f64::INFINITY, f64::NEG_INFINITY].iter() {
            let mut b = 0.;
            let a = unsafe { modf(*x, &mut b)};
            let (c, d) = libm::modf(*x);
            if  !crate::equalf64(a, c) || !crate::equalf64(b, d) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
        let mut r = rand::thread_rng();
        for _ in 0..500 {
            let mut b = 0.;
            let x = r.gen::<f64>();
            let a = unsafe { modf(x, &mut b)};
            let (c, d) = libm::modf(x);
            if  !crate::equalf64(a, c) || !crate::equalf64(b, d) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn modff(x: f32, y: *mut f32) -> f32; }

    #[test]
    pub fn modff_matches() {
        for x in [f32::NAN, -f32::NAN, f32::INFINITY, f32::NEG_INFINITY].iter() {
            let mut b = 0.;
            let a = unsafe { modff(*x, &mut b) };
            let (c, d) = libm::modff(*x);
            if  !crate::equalf32(a, c) || !crate::equalf32(b, d) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
        let mut r = rand::thread_rng();
        for _ in 0..500 {
            let mut b = 0.;
            let x = r.gen::<f32>();
            let a = unsafe { modff(x, &mut b) };
            let (c, d) = libm::modff(x);
            if  !crate::equalf32(a, c) || !crate::equalf32(b, d) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn remquo(x: f64, y: f64, b: *mut i32) -> f64; }

    #[test]
    pub fn remquo_matches() {
        let mut r = rand::thread_rng();
        for _ in 0..500 {
            let mut b = 0;
            let x = r.gen::<f64>();
            let y = r.gen::<f64>();
            let a = unsafe { remquo(x, y, &mut b)};
            let (c, d) = libm::remquo(x, y);
            if  !crate::equalf64(a, c) || !crate::equali32(b, d) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn remquof(x: f32, y: f32, b: *mut i32) -> f32; }

    #[test]
    pub fn remquof_matches() {
        let mut r = rand::thread_rng();
        for _ in 0..500 {
            let mut b = 0;
            let x = r.gen::<f32>();
            let y = r.gen::<f32>();
            let a = unsafe { remquof(x, y, &mut b)};
            let (c, d) = libm::remquof(x, y);
            if  !crate::equalf32(a, c) || !crate::equali32(b, d)  {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn frexp(x: f64, y: *mut i32) -> f64; }

    #[test]
    pub fn frexp_matches() {
        let mut r = rand::thread_rng();
        for x in [f64::NAN, -f64::NAN, f64::INFINITY, f64::NEG_INFINITY].iter() {
            let mut b = r.gen::<i32>();
            let a  = unsafe { frexp(*x, &mut b) };
            let (c, d) = libm::frexp(*x);
            if  !crate::equalf64(a, c) || !crate::equali32(b, d) {
              panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
        for _ in 0..500 {
          let x = r.gen::<f64>();
          let mut b = r.gen::<i32>();
          let a  = unsafe { frexp(x, &mut b) };
          let (c, d) = libm::frexp(x);
          if  !crate::equalf64(a, c) || !crate::equali32(b, d) {
              panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
          }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn frexpf(x: f32, y: *mut i32) -> f32; }

    #[test]
    pub fn frexpf_matches() {
        let mut r = rand::thread_rng();
        for x in [f32::NAN, -f32::NAN, f32::INFINITY, f32::NEG_INFINITY].iter() {
            let mut b = 0;
            let a  = unsafe { frexpf(*x, &mut b) };
            let (c, d) = libm::frexpf(*x);
            if  !crate::equalf32(a, c) || !crate::equali32(b, d) {
              panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
        for _ in 0..500 {
            let x = r.gen::<f32>();
            let mut b = 0;
            let a  = unsafe { frexpf(x, &mut b) };
            let (c, d) = libm::frexpf(x);
            if  !crate::equalf32(a, c) || !crate::equali32(b, d) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (a, b), (c, d));
            }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn sincos(x: f64, sin: *mut f64, cos: *mut f64); }

    #[test]
    pub fn sincos_matches() {
        for x in [f64::NAN, -f64::NAN, f64::INFINITY, f64::NEG_INFINITY].iter() {
            let mut sin = 0.;
            let mut cos = 0.;
            unsafe { sincos(*x, &mut sin, &mut cos) };
            let result = libm::sincos(*x);
            if  !crate::equalf64(sin, result.0) || !crate::equalf64(cos, result.1) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (sin, cos), result);
            }
        }

        let mut r = rand::thread_rng();
        for _ in 0..500 {
            let x = r.gen::<f64>();
            let mut sin = 0.;
            let mut cos = 0.;
            unsafe { sincos(x, &mut sin, &mut cos) };
            let result = libm::sincos(x);
            if  !crate::equalf64(sin, result.0) || !crate::equalf64(cos, result.1) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (sin, cos), result);
            }
        }
    }

    #[cfg(test)]
    extern "C" { pub fn sincosf(x: f32, sin: *mut f32, cos: *mut f32); }

    #[test]
    pub fn sincosf_matches() {
        for x in [f32::NAN, -f32::NAN, f32::INFINITY, f32::NEG_INFINITY].iter() {
            let mut sin = 0.;
            let mut cos = 0.;
            unsafe { sincosf(*x, &mut sin, &mut cos) };
            let result = libm::sincosf(*x);
            if  !crate::equalf32(sin, result.0) || !crate::equalf32(cos, result.1) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (sin, cos), result);
            }
        }

        let mut r = rand::thread_rng();
        for _ in 0..500 {
          let x = r.gen::<f32>();
            let mut sin = 0.;
            let mut cos = 0.;
            unsafe { sincosf(x, &mut sin, &mut cos) };
            let result = libm::sincosf(x);
            if  !crate::equalf32(sin, result.0) || !crate::equalf32(cos, result.1) {
                panic!("INPUT: {:?} EXPECTED: {:?} ACTUAL {:?}", x, (sin, cos), result);
            }
        }
    }
}
