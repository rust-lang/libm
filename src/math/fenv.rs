// src: musl/src/fenv/fenv.c
/* Dummy functions for archs lacking fenv implementation */

pub(crate) const FE_UNDERFLOW: i32 = 0;
pub(crate) const FE_INEXACT: i32 = 0;

pub(crate) const FE_TONEAREST: i32 = 0;

#[inline]
pub(crate) const fn feclearexcept(_mask: i32) -> i32 {
    0
}

#[inline]
pub(crate) const fn feraiseexcept(_mask: i32) -> i32 {
    0
}

#[inline]
pub(crate) const fn fetestexcept(_mask: i32) -> i32 {
    0
}

#[inline]
pub(crate) const fn fegetround() -> i32 {
    FE_TONEAREST
}
