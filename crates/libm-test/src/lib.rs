//! Testing utilities required by most tests.

mod within_ulps;
pub use self::within_ulps::WithinUlps;

mod call_fn;
pub use self::call_fn::CallFn;

mod tuple_vec;
pub use self::tuple_vec::TupleVec;

mod api_kind;
pub use self::api_kind::ApiKind;

mod float_rng;
pub use self::float_rng::{Distribute, Toward};

mod rand_seq;
pub use self::rand_seq::RandSeq;

/// Asserts that two values are approximately equal up-to ULP tolerance
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
