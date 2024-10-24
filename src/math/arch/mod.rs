cfg_if! {
    if #[cfg(all(target_arch = "x86", not(target_feature = "sse2")))] {
        mod i586;
        pub use i586::ceil::ceil;
        pub use i586::floor::floor;
    } else if #[cfg(target_feature = "sse2")] {
        mod i686;
        pub use i686::sqrt::sqrt;
        pub use i686::sqrtf::sqrtf;
    } else if #[cfg(all(target_arch = "wasm32", feature = "unstable"))] {
        mod intrinsic;
        pub use intrinsic::ceil::ceil;
        pub use intrinsic::ceilf::ceilf;
        pub use intrinsic::fabs::fabs;
        pub use intrinsic::fabsf::fabsf;
        pub use intrinsic::floor::floor;
        pub use intrinsic::floorf::floorf;
        pub use intrinsic::sqrt::sqrt;
        pub use intrinsic::sqrtf::sqrtf;
        pub use intrinsic::trunc::trunc;
        pub use intrinsic::truncf::truncf;
    }
}
