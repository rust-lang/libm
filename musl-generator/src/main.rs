extern crate libm;
extern crate shared;

use std::error::Error;
use std::fs::File;
use std::io::Write;

#[macro_use]
mod macros;

fn main() -> Result<(), Box<Error>> {
    f32! {
        acosf,
        acoshf,
        asinf,
        asinhf,
        atanf,
        atanhf,
        cbrtf,
        ceilf,
        cosf,
        coshf,
        erff,
        erfcf,
        exp10f,
        exp2f,
        expf,
        expm1f,
        fabsf,
        floorf,
        j0f,
        j1f,
        lgammaf,
        log10f,
        log1pf,
        log2f,
        logf,
        roundf,
        sinf,
        sinhf,
        sqrtf,
        tanf,
        tanhf,
        tgammaf,
        truncf,
        y0f,
        y1f,
    }

    f32f32! {
        atan2f,
        copysignf,
        fdimf,
        fmodf,
        hypotf,
        powf,
    }

    f32i32! {
        scalbnf,
    }

    f32f32f32! {
        fmaf,
    }

    f64! {
        acos,
        acosh,
        asin,
        asinh,
        atan,
        atanh,
        cbrt,
        ceil,
        cos,
        cosh,
        exp,
        exp10,
        exp2,
        expm1,
        erf,
        erfc,
        fabs,
        floor,
        j0,
        j1,
        lgamma,
        log,
        log10,
        log1p,
        log2,
        round,
        sin,
        sinh,
        sqrt,
        tan,
        tanh,
        tgamma,
        trunc,
        y0,
        y1,
    }

    f64f64! {
        atan2,
        copysign,
        fdim,
        fmod,
        hypot,
        pow,
    }

    f64i32! {
        scalbn,
    }

    f64f64f64! {
        fma,
    }

    Ok(())
}
