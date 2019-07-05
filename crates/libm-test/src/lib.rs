//! Testing utilities required by most tests.

pub trait WithinUlps {
    /// Returns true if two numbers are closer than `ulp_tol` to each other.
    fn within_ulps(self, other: Self, ulp_tol: usize) -> bool;
}

// Stamp the impls for floats:
macro_rules! impl_within_ulps_f {
    ($f_ty:ty, $i_ty:ty) => {
        impl WithinUlps for $f_ty {
            #[allow(clippy::cast_possible_truncation)]
            fn within_ulps(self, y: Self, ulp_tol: usize) -> bool {
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

                let xi: $i_ty = unsafe { core::intrinsics::transmute(x) };
                let yi: $i_ty = unsafe { core::intrinsics::transmute(y) };
                if (xi < 0) != (yi < 0) {
                    // different sign, e.g., -0.0 != +0.0:
                    return false;
                }
                let ulps = (xi - yi).abs();
                ulps <= ulp_tol as _
            }
        }
    };
}

impl_within_ulps_f!(f32, i32);
impl_within_ulps_f!(f64, i64);

impl WithinUlps for i32 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn within_ulps(self, y: Self, ulp_tol: usize) -> bool {
        let x = self;
        let ulps = (x - y).abs();
        ulps <= ulp_tol as _
    }
}

/// This implements function dispatch for tuples of arguments used in the tests
/// above, so that we can: (f32, 32).call(fn(f32, f32) -> f32) generically.
///
/// We need the input parameter F to support dispatching, e.g., (f32,f32) with
/// functions that return both f32 or i32. Those are two different types, so we
/// need to be parametric over them.
pub trait Call<F> {
    type Ret;
    fn call(self, f: F) -> Self::Ret;
}

macro_rules! impl_call {
    (($($arg_tys:ty),*) -> $ret_ty:ty: $self_:ident: $($xs:expr),*)  => {
        impl Call<unsafe extern"C" fn($($arg_tys),*) -> $ret_ty> for ($($arg_tys,)+) {
            type Ret = $ret_ty;
            fn call(self, f: unsafe extern "C" fn($($arg_tys),*) -> $ret_ty) -> Self::Ret {
                let $self_ = self;
                unsafe { f($($xs),*) }
            }
        }
    };
}

impl_call!((f32) -> f32: x: x.0);
impl_call!((f64) -> f64: x: x.0);
impl_call!((f64) -> i32: x: x.0);
impl_call!((f32) -> i32: x: x.0);
impl_call!((f32, f32) -> f32: x: x.0, x.1);
impl_call!((f32, f64) -> f32: x: x.0, x.1);
impl_call!((f64, f64) -> f64: x: x.0, x.1);
impl_call!((f64, i32) -> f64: x: x.0, x.1);
impl_call!((f32, i32) -> f32: x: x.0, x.1);
impl_call!((i32, f64) -> f64: x: x.0, x.1);
impl_call!((i32, f32) -> f32: x: x.0, x.1);
impl_call!((f32, f32, f32) -> f32: x: x.0, x.1, x.2);
impl_call!((f64, f64, f64) -> f64: x: x.0, x.1, x.2);

pub trait TupleVec {
    type Output;
    fn get(&self, i: usize) -> Self::Output;
}

macro_rules! impl_tuple_vec {
    (($($arg_tys:ty),*): $self_:ident: $($xs:expr),*)  => {
        impl TupleVec for ($(Vec<$arg_tys>,)+) {
            type Output = ($($arg_tys,)+);
            fn get(&self, i: usize) -> Self::Output {
                let $self_ = self;
                ($($xs[i],)*)
            }
        }
    };
}

impl_tuple_vec!((f32): x: x.0);
impl_tuple_vec!((f64): x: x.0);
impl_tuple_vec!((f32, f32): x: x.0, x.1);
impl_tuple_vec!((f32, f64): x: x.0, x.1);
impl_tuple_vec!((f64, f64): x: x.0, x.1);
impl_tuple_vec!((f64, i32): x: x.0, x.1);
impl_tuple_vec!((f32, i32): x: x.0, x.1);
impl_tuple_vec!((i32, f64): x: x.0, x.1);
impl_tuple_vec!((i32, f32): x: x.0, x.1);
impl_tuple_vec!((f32, f32, f32): x: x.0, x.1, x.2);
impl_tuple_vec!((f64, f64, f64): x: x.0, x.1, x.2);

/// Kind of libm API - used to handle generating tests
/// for some functions slightly differently.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ApiKind {
    Jx,
    Other,
}

#[macro_export]
macro_rules! get_api_kind {
    (fn: j1) => {
        $crate::ApiKind::Jx
    };
    (fn: jn) => {
        $crate::ApiKind::Jx
    };
    (fn: $id:ident) => {
        $crate::ApiKind::Other
    };
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($result:ident == $expected:ident,
     id: $id:ident, arg: $arg:ident, ulp: $ulps:expr) => {
        if !$crate::WithinUlps::within_ulps($result, $expected, $ulps) {
            let f = format!(
                "{}{:?} returns = {:?} != {:?} (expected)",
                stringify!($id),
                $arg,
                $result,
                $expected
            );
            panic!(f);
        }
    };
    ($result:expr, $expected:expr, ulp: $ulps:expr) => {
        if !$crate::WithinUlps::within_ulps($result, $expected, $ulps) {
            panic!("{:?} != {:?}", $result, $expected);
        }
    };
}

pub trait Toward: Sized {
    fn toward(self, other: Self, len: usize) -> Vec<Self>;
}

macro_rules! impl_toward_f {
    ($float_ty:ident, $toward_fn:path) => {
        impl Toward for $float_ty {
            fn toward(self, other: Self, len: usize) -> Vec<Self> {
                let mut vec = Vec::with_capacity(len);
                let mut current = self;
                vec.push(self);
                for _ in 0..=len {
                    current = $toward_fn(current, other as _);
                    vec.push(self);
                    if current.to_bits() == other.to_bits() {
                        break;
                    }
                }
                vec
            }
        }
    };
}
impl_toward_f!(f32, libm::nextafterf);
impl_toward_f!(f64, libm::nextafter);

pub trait RandSeq: Sized {
    fn rand_seq<R: rand::Rng>(rng: &mut R, api_kind: ApiKind, len: usize) -> Vec<Self>;
}

macro_rules! impl_rand_seq_f {
    ($float_ty:ident) => {
        #[allow(clippy::use_self)]
        impl RandSeq for $float_ty {
            fn rand_seq<R: rand::Rng>(rng: &mut R, _api_kind: ApiKind, len: usize) -> Vec<Self> {
                use rand::seq::SliceRandom;
                use std::$float_ty::*;
                let mut vec = Vec::with_capacity(len);

                // These inputs are always tested
                const BOUNDS: [$float_ty; 9] = [
                    NAN,
                    INFINITY,
                    NEG_INFINITY,
                    EPSILON,
                    -EPSILON,
                    MAX,
                    MIN,
                    MIN_POSITIVE,
                    -MIN_POSITIVE,
                ];
                vec.extend(&BOUNDS);
                // A range around the inputs is also always tested:
                const NSTEPS: usize = 1_000;
                vec.extend(INFINITY.toward(0., NSTEPS));
                vec.extend(NEG_INFINITY.toward(0., NSTEPS));
                vec.extend((0. as Self).toward(MIN_POSITIVE, NSTEPS));
                vec.extend((0. as Self).toward(-MIN_POSITIVE, NSTEPS));

                for i in 0..=NSTEPS {
                    let dx = 2. / NSTEPS as Self;
                    let next = (-1. as Self) + (i as Self) * dx;
                    vec.push(next);
                }

                // ~NSTEPS * 4
                assert!(len > 2 * 4 * NSTEPS, "len {} !> {}", len, 2 * 4 * NSTEPS);
                let current_len = vec.len();
                let remaining_len = len.checked_sub(current_len).unwrap();

                for _ in 0..remaining_len {
                    let n = rng.gen::<Self>();
                    vec.push(n);
                }
                assert_eq!(vec.len(), len);

                // Duplicate the vector, randomly shuffle it, and
                // concatenate it. Otherwise for n-ary functions
                // all vectors might have the same values. But
                // testing with the same values is also worth doing.
                let mut vec2 = vec.clone();
                vec2.shuffle(rng);
                vec.extend(vec2);
                vec
            }
        }
    };
}

impl_rand_seq_f!(f32);
impl_rand_seq_f!(f64);

impl RandSeq for i32 {
    fn rand_seq<R: rand::Rng>(rng: &mut R, api_kind: ApiKind, len: usize) -> Vec<Self> {
        use rand::seq::SliceRandom;
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            let mut r = rng.gen::<Self>();
            if let ApiKind::Jx = api_kind {
                r &= 0xffff;
            }
            v.push(r);
        }
        assert_eq!(v.len(), len);

        // Duplicate the vector, randomly shuffle it, and
        // concatenate it. Otherwise for n-ary functions
        // all vectors might have the same values. But
        // testing with the same values is also worth doing.
        let mut v2 = v.clone();
        v2.shuffle(rng);
        v.extend(v2);
        v
    }
}
