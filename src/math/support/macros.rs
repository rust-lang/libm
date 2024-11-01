/// `libm` cannot have dependencies, so this is vendored directly from the `cfg-if` crate
/// (with some comments stripped for compactness).
macro_rules! cfg_if {
    // match if/else chains with a final `else`
    ($(
        if #[cfg($meta:meta)] { $($tokens:tt)* }
    ) else * else {
        $($tokens2:tt)*
    }) => {
        cfg_if! { @__items () ; $( ( ($meta) ($($tokens)*) ), )* ( () ($($tokens2)*) ), }
    };

    // match if/else chains lacking a final `else`
    (
        if #[cfg($i_met:meta)] { $($i_tokens:tt)* }
        $( else if #[cfg($e_met:meta)] { $($e_tokens:tt)* } )*
    ) => {
        cfg_if! {
            @__items
            () ;
            ( ($i_met) ($($i_tokens)*) ),
            $( ( ($e_met) ($($e_tokens)*) ), )*
            ( () () ),
        }
    };

    // Internal and recursive macro to emit all the items
    //
    // Collects all the negated cfgs in a list at the beginning and after the
    // semicolon is all the remaining items
    (@__items ($($not:meta,)*) ; ) => {};
    (@__items ($($not:meta,)*) ; ( ($($m:meta),*) ($($tokens:tt)*) ), $($rest:tt)*) => {
        #[cfg(all($($m,)* not(any($($not),*))))] cfg_if! { @__identity $($tokens)* }
        cfg_if! { @__items ($($not,)* $($m,)*) ; $($rest)* }
    };

    // Internal macro to make __apply work out right for different match types,
    // because of how macros matching/expand stuff.
    (@__identity $($tokens:tt)*) => { $($tokens)* };
}

/// Choose among using an intrinsic, an arch-specific implementation, and the function body.
/// Returns directly if the intrinsic or arch is used, otherwise continue with the rest of the
/// function.
///
/// Specify a `use_intrinsic` meta field if the intrinsic is (1) available on the platforms (i.e.
/// LLVM lowers it without libcalls that may recurse), (2) it is likely to be more performant.
/// Intrinsics require wrappers in the `math::arch::intrinsics` module.
///
/// Specify a `use_arch` meta field if an architecture-specific implementation is provided.
/// These live in the `math::arch::some_target_arch` module.
///
/// Specify a `use_arch_required` meta field if something architecture-specific must be used
/// regardless of feature configuration (`force-soft-floats`).
///
/// The passed meta options do not need to account for relevant Cargo features
/// (`unstable-intrinsics`, `arch`, `force-soft-floats`), this macro handles that part.
macro_rules! select_implementation {
    (
        name: $fn_name:ident,
        // Configuration meta for when to use arch-specific implementation that requires hard
        // float ops
        $( use_arch: $use_arch:meta, )?
        // Configuration meta for when to use the arch module regardless of whether softfloats
        // have been requested.
        $( use_arch_required: $use_arch_required:meta, )?
        // Configuration meta for when to call intrinsics and let LLVM figure it out
        $( use_intrinsic: $use_intrinsic:meta, )?
        args: $($arg:ident),+ ,
    ) => {
        // FIXME: these use paths that are a pretty fragile (`super`). We should figure out
        // something better w.r.t. how this is vendored into compiler-builtins.

        // However, we do need a few things from `arch` that are used even with soft floats.
        //
        select_implementation! {
            @cfg $($use_arch_required)?;
            if true {
                return  super::arch::$fn_name( $($arg),+ );
            }
        }

        // By default, never use arch-specific implementations if we have force-soft-floats
        #[cfg(arch_enabled)]
        select_implementation! {
            @cfg $($use_arch)?;
            // Wrap in `if true` to avoid unused warnings
            if true {
                return  super::arch::$fn_name( $($arg),+ );
            }
        }

        // Never use intrinsics if we are forcing soft floats, and only enable with the
        // `unstable-intrinsics` feature.
        #[cfg(intrinsics_enabled)]
        select_implementation! {
            @cfg $( $use_intrinsic )?;
            if true {
                return  super::arch::intrinsics::$fn_name( $($arg),+ );
            }
        }
    };

    // Coalesce helper to construct an expression only if a config is provided
    (@cfg ; $ex:expr) => { };
    (@cfg $provided:meta; $ex:expr) => { #[cfg($provided)] $ex };
}

/// Construct a 32-bit float from hex float representation (C-style), guaranteed to
/// evaluate at compile time.
#[allow(unused_macros)]
macro_rules! hf32 {
    ($s:literal) => {{
        const X: f32 = $crate::math::support::hf32($s);
        X
    }};
}

/// Construct a 64-bit float from hex float representation (C-style), guaranteed to
/// evaluate at compile time.
#[allow(unused_macros)]
macro_rules! hf64 {
    ($s:literal) => {{
        const X: f64 = $crate::math::support::hf64($s);
        X
    }};
}
