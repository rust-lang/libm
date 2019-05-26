macro_rules! llvm_intrinsically_optimized {
    (#[cfg($($clause:tt)*)] $e:expr) => {
        #[cfg(all(not(feature = "stable"), $($clause)*))]
        {
            if true { // thwart the dead code lint
                $e
            }
        }
    };
}

mod sqrtf;

pub use self::sqrtf::sqrtf;
