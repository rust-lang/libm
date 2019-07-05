#![feature(test)]
extern crate test;

use libm_test::{get_api_kind, ApiKind, Call};
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
            type FnTy
                = unsafe extern "C" fn ($($arg_ids: $arg_tys),*) -> $ret_ty;

            // Generate a tuple of arguments containing random values:
            let mut rng = rand::thread_rng();
            let mut x: ( $($arg_tys,)+ ) = ( $(rng.gen::<$arg_tys>(),)+ );

            if let ApiKind::Jx = get_api_kind!(fn: $id) {
                let ptr = &mut x as *mut _ as *mut i32;
                unsafe { ptr.write(ptr.read() & 0xffff) };
            }

            bh.iter(|| test::black_box(x).call(libm::$id as FnTy))
        }
    };
}

libm_analyze::for_each_api!(bench_fn);
