//! Architecture-specific support for aarch64 with neon.

use core::arch::asm;

pub fn ceil(mut x: f64) -> f64 {
    unsafe {
        asm!(
            "frintp {x:d}, {x:d}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn ceilf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "frintp {x:s}, {x:s}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

#[cfg(all(f16_enabled, target_feature = "fp16"))]
pub fn ceilf16(mut x: f16) -> f16 {
    unsafe {
        asm!(
            "frintp {x:h}, {x:h}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn fabs(mut x: f64) -> f64 {
    unsafe {
        asm!(
            "fabs {x:d}, {x:d}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn fabsf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "fabs {x:s}, {x:s}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

#[cfg(all(f16_enabled, target_feature = "fp16"))]
pub fn fabsf16(mut x: f16) -> f16 {
    unsafe {
        asm!(
            "fabs {x:h}, {x:h}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn floor(mut x: f64) -> f64 {
    unsafe {
        asm!(
            "frintm {x:d}, {x:d}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn floorf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "frintm {x:s}, {x:s}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

#[cfg(all(f16_enabled, target_feature = "fp16"))]
pub fn floorf16(mut x: f16) -> f16 {
    unsafe {
        asm!(
            "frintm {x:h}, {x:h}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn fma(mut x: f64, y: f64, z: f64) -> f64 {
    unsafe {
        asm!(
            "fmadd {x:d}, {x:d}, {y:d}, {z:d}",
            x = inout(vreg) x,
            y = in(vreg) y,
            z = in(vreg) z,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn fmaf(mut x: f32, y: f32, z: f32) -> f32 {
    unsafe {
        asm!(
            "fmadd {x:s}, {x:s}, {y:s}, {z:s}",
            x = inout(vreg) x,
            y = in(vreg) y,
            z = in(vreg) z,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn rint(mut x: f64) -> f64 {
    unsafe {
        asm!(
            "frinti {x:d}, {x:d}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn rintf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "frinti {x:s}, {x:s}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

#[cfg(all(f16_enabled, target_feature = "fp16"))]
pub fn rintf16(mut x: f16) -> f16 {
    unsafe {
        asm!(
            "frinti {x:h}, {x:h}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn round(mut x: f64) -> f64 {
    unsafe {
        asm!(
            "frinta {x:d}, {x:d}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn roundf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "frinta {x:s}, {x:s}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

#[cfg(all(f16_enabled, target_feature = "fp16"))]
pub fn roundf16(mut x: f16) -> f16 {
    unsafe {
        asm!(
            "frinta {x:h}, {x:h}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn sqrt(mut x: f64) -> f64 {
    unsafe {
        asm!(
            "fsqrt {x:d}, {x:d}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn sqrtf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "fsqrt {x:s}, {x:s}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

#[cfg(all(f16_enabled, target_feature = "fp16"))]
pub fn sqrtf16(mut x: f16) -> f16 {
    unsafe {
        asm!(
            "fsqrt {x:h}, {x:h}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn trunc(mut x: f64) -> f64 {
    unsafe {
        asm!(
            "frintz {x:d}, {x:d}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

pub fn truncf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "frintz {x:s}, {x:s}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}

#[cfg(all(f16_enabled, target_feature = "fp16"))]
pub fn truncf16(mut x: f16) -> f16 {
    unsafe {
        asm!(
            "frintz {x:h}, {x:h}",
            x = inout(vreg) x,
            options(nomem, nostack, pure)
        );
    }
    x
}
