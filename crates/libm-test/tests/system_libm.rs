//! Compare the results of the `libm` implementation against the system's libm.
#![cfg(test)]

// Number of tests to generate for each function
const NTESTS: usize = 500;

// FIXME: should be 1
const ULP_TOL: usize = 3;

macro_rules! system_libm {
    (
        id: $id:ident;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ret_ty:ty;
    ) => {
        #[test]
        #[allow(unused)]
        fn $id() {
            use crate::Call;
            let mut rng = rand::thread_rng();
            for _ in 0..NTESTS {
                let args: ( $($arg_tys),+ ) = ( $(<$arg_tys as Rand>::gen(&mut rng)),+ );
                extern "C" fn libm_fn($($arg_ids: $arg_tys),*) -> $ret_ty {
                    libm::$id($($arg_ids),*)
                }
                let result = args.call(libm_fn);
                extern "C" {
                    fn $id($($arg_ids: $arg_tys),*) -> $ret_ty;
                }
                let expected = args.call($id);
                if !result.eq(expected) {
                    eprintln!("result = {} != {} (expected)", result, expected);
                    panic!();
                }
            }
        }
    }
}

libm_analyze::for_each_api!(system_libm);

trait Call<F> {
    type Ret;
    fn call(self, f: F) -> Self::Ret;
}


macro_rules! impl_call {
    (($($arg_tys:ty),*) -> $ret_ty:ty: $self_:ident: $($xs:expr),*)  => {
        impl Call<unsafe extern"C" fn($($arg_tys),*) -> $ret_ty> for ($($arg_tys),+) {
            type Ret = $ret_ty;
            fn call(self, f: unsafe extern "C" fn($($arg_tys),*) -> $ret_ty) -> Self::Ret {
                let $self_ = self;
                unsafe { f($($xs),*) }
            }
        }
    };
}

impl_call!((f32) -> f32: x: x);
impl_call!((f32,f32) -> f32: x: x.0, x.1);
impl_call!((f32,f32,f32) -> f32: x: x.0, x.1, x.2);
impl_call!((f64) -> f64: x: x);
impl_call!((f64,f64) -> f64: x: x.0, x.1);
impl_call!((f64,f64,f64) -> f64: x: x.0, x.1, x.2);

impl_call!((f64, i32) -> f64: x: x.0, x.1);
impl_call!((f32, i32) -> f32: x: x.0, x.1);

trait Rand {
    fn gen(rng: &mut rand::rngs::ThreadRng) -> Self;
}

macro_rules! impl_rand {
    ($id:ident: [$($e:expr),*]) => {
        impl Rand for $id {
            fn gen(r: &mut rand::rngs::ThreadRng) -> Self {
                use rand::Rng;
                use rand::seq::SliceRandom;
                let r = if r.gen_range(0, 20) < 1 {
                    *[$($e),*].choose(r).unwrap()
                } else {
                    r.gen::<$id>()
                };
                unsafe { std::mem::transmute(r) }
            }
        }
    }
}

impl_rand!(f32: [std::f32::NAN, std::f32::INFINITY, std::f32::NEG_INFINITY]);
impl_rand!(f64: [std::f64::NAN, std::f64::INFINITY, std::f64::NEG_INFINITY]);
impl_rand!(i32: [i32::max_value(), 0_i32, i32::min_value()]);

trait Equal {
    fn eq(self, other: Self) -> bool;
}

macro_rules! impl_eq_f {
    ($f_ty:ty, $i_ty:ty) => {
        impl Equal for $f_ty {
            fn eq(self, y: $f_ty) -> bool {
                let x = self;
                if x.is_nan() != y.is_nan() {
                    // one is nan but the other is not
                    return false;
                }
                if x.is_nan() && y.is_nan() {
                    return true;
                }
                if x.is_infinite() != y.is_infinite() {
                    // one is inf but the other is not
                    return false;
                }
                if x.is_infinite() != y.is_infinite() {
                    // one is inf but the other is not
                    return false;
                }
                let xi: $i_ty = unsafe { core::intrinsics::transmute(x) };
                let yi: $i_ty = unsafe { core::intrinsics::transmute(y) };
                if (xi < 0) != (yi < 0) {
                    // different sign
                    return false;
                }
                let ulps = (xi - yi).abs();
                ulps <= ULP_TOL as _
            }
        }
    }
}

impl_eq_f!(f32, i32);
impl_eq_f!(f64, i64);

impl Equal for i32 {
    fn eq(self, y: i32) -> bool {
        let x = self;
        let ulps = (x - y).abs();
        ulps <= 1
    }
}
