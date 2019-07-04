//! Testing utilities required by most tests.

pub trait WithinUlps {
    /// Returns true if two numbers are closer than `ulp_tol` to each other.
    fn within_ulps(self, other: Self, ulp_tol: usize) -> bool;
}

// TODO: this should be moved to the libm-test/src/lib.rs library. And we
// should make ulps configurable.

// Stamp the impls for floats:
macro_rules! impl_within_ulps_f {
    ($f_ty:ty, $i_ty:ty) => {
        impl WithinUlps for $f_ty {
            fn within_ulps(self, y: $f_ty, ulp_tol: usize) -> bool {
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
    fn within_ulps(self, y: i32, ulp_tol: usize) -> bool {
        let x = self;
        let ulps = (x - y).abs();
        ulps <= ulp_tol as _
    }
}
