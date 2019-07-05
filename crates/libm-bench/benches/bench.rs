#![feature(test)]
extern crate test;

use libm_test::{adjust_input, Call};
use rand::Rng;
use test::Bencher;

macro_rules! bench_fn {
    // FIXME: jnf benches never terminate
    (
        id: jnf;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) => {};
    (
        id: $id:ident;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret_ty: $ret_ty:ty;
    ) => {
        #[bench]
        #[allow(unused_mut)]
        pub fn $id(bh: &mut Bencher) {
            // Type of the system libm fn:
            type FnTy = unsafe extern "C" fn ($($arg_ids: $arg_tys),*) -> $ret_ty;

            // FIXME: extern "C" wrapper
            extern "C" fn libm_fn($($arg_ids: $arg_tys),*) -> $ret_ty {
                libm::$id($($arg_ids),*)
            }

            // Generate a tuple of arguments containing random values:
            let mut rng = rand::thread_rng();
            let mut x: ( $($arg_tys,)+ ) = ( $(rng.gen::<$arg_tys>(),)+ );

            adjust_input!(fn: $id, input: x);

            bh.iter(|| test::black_box(x).call(libm_fn as FnTy))
        }
    };
}

libm_analyze::for_each_api!(bench_fn);
