//! Traits and operations related to bounds of a function.

use std::fmt;
use std::ops::{self, Bound};

use crate::Float;

/// A trait to be implemented on types representing a function's domain.
///
/// Since multiple functions share the same domain, this doesn't get implemented on the `op::*`
/// type. Instead, this gets applied to a new unit struct, then the `op::*` type should
/// implement `HasDomain`.
pub struct Domain<T> {
    /// The region for which the function is defined. Ignores poles.
    pub start: Bound<T>,
    pub end: Bound<T>,
    /// Additional points to check closer around. These can be e.g. undefined asymptotes or
    /// inflection points.
    pub check_points: Option<fn() -> Box<dyn Iterator<Item = T>>>,
}

/// Possible domains
impl<F: Float> Domain<F> {
    /// x ∈ ℝ
    pub const UNBOUNDED: Self =
        Self { start: Bound::Unbounded, end: Bound::Unbounded, check_points: None };

    /// x ∈ ℝ >= 0
    pub const POSITIVE: Self =
        Self { start: Bound::Included(F::ZERO), end: Bound::Unbounded, check_points: None };

    /// x ∈ ℝ > 0
    pub const STRICTLY_POSITIVE: Self =
        Self { start: Bound::Excluded(F::ZERO), end: Bound::Unbounded, check_points: None };

    /// Used for versions of `asin` and `acos`.
    pub const INVERSE_TRIG_PERIODIC: Self = Self {
        start: Bound::Included(F::NEG_ONE),
        end: Bound::Included(F::ONE),
        check_points: None,
    };

    /// Domain for `acosh`
    pub const ACOSH: Self =
        Self { start: Bound::Included(F::ONE), end: Bound::Unbounded, check_points: None };

    /// Domain for `atanh`
    pub const ATANH: Self = Self {
        start: Bound::Excluded(F::NEG_ONE),
        end: Bound::Excluded(F::ONE),
        check_points: None,
    };

    /// Domain for `sin`, `cos`, and `tan`
    pub const TRIG: Self = Self {
        // TODO
        check_points: None,
        ..Self::UNBOUNDED
    };

    /// Domain for `log` in various bases
    pub const LOG: Self = Self::STRICTLY_POSITIVE;

    /// Domain for `log1p` i.e. `log(1 + x)`
    pub const LOG1P: Self =
        Self { start: Bound::Excluded(F::NEG_ONE), end: Bound::Unbounded, check_points: None };

    /// Domain for `sqrt`
    pub const SQRT: Self = Self::POSITIVE;
}

/// Implement on `op::*` types to indicate how they are bounded.
pub trait HasDomain<T>
where
    T: Copy + fmt::Debug + ops::Add<Output = T> + ops::Sub<Output = T> + PartialOrd + 'static,
{
    const D: Domain<T>;
}

/// Implement [`HasDomain`] for both the `f32` and `f64` variants of a function.
macro_rules! impl_has_domain {
    ($($fn_name:ident => $domain:expr;)*) => {
        paste::paste! {
            $(
                // Implement for f64 functions
                impl HasDomain<f64> for $crate::op::$fn_name::Routine {
                    const D: Domain<f64> = Domain::<f64>::$domain;
                }

                // Implement for f32 functions
                impl HasDomain<f32> for $crate::op::[< $fn_name f >]::Routine {
                    const D: Domain<f32> = Domain::<f32>::$domain;
                }
            )*
        }
    };
}

// Tie functions together with their domains.
impl_has_domain! {
    acos => INVERSE_TRIG_PERIODIC;
    acosh => ACOSH;
    asin => INVERSE_TRIG_PERIODIC;
    asinh => UNBOUNDED;
    atan => UNBOUNDED;
    atanh => ATANH;
    cbrt => UNBOUNDED;
    ceil => UNBOUNDED;
    cos => TRIG;
    cosh => UNBOUNDED;
    erf => UNBOUNDED;
    exp => UNBOUNDED;
    exp10 => UNBOUNDED;
    exp2 => UNBOUNDED;
    expm1 => UNBOUNDED;
    fabs => UNBOUNDED;
    floor => UNBOUNDED;
    frexp => UNBOUNDED;
    j0 => UNBOUNDED;
    j1 => UNBOUNDED;
    log => LOG;
    log10 => LOG;
    log1p => LOG1P;
    log2 => LOG;
    rint => UNBOUNDED;
    round => UNBOUNDED;
    sin => TRIG;
    sinh => UNBOUNDED;
    sqrt => SQRT;
    tan => TRIG;
    tanh => UNBOUNDED;
    trunc => UNBOUNDED;
}
