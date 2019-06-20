#![cfg(test)]
extern crate core;
extern crate libm;
extern crate paste;
extern crate rand;

pub mod conformance;

#[cfg(target_arch = "x86_64")]
pub mod validation;

#[cfg(test)]
pub fn equalf64(x: f64, y: f64) -> bool {
    if x.is_nan() != y.is_nan() {
        // one is nan but the other is not
        return false;
    }
    if x.is_nan() && y.is_nan() {
        return true;
    }
    if x.is_infinite() != y.is_infinite() {
        // one is inf but the other is not
        return false;
    }
    if x.is_infinite() != y.is_infinite() {
        // one is inf but the other is not
        return false;
    }
    let xi: i64 = unsafe { core::intrinsics::transmute(x) };
    let yi: i64 = unsafe { core::intrinsics::transmute(y) };
    if (xi < 0) != (yi < 0) {
        // different sign
        return false;
    }
    let ulps = (xi - yi).abs();
    ulps <= 1
}

#[cfg(test)]
pub fn equalf32(x: f32, y: f32) -> bool {
    if x.is_nan() != y.is_nan() {
        // one is nan but the other is not
        return false;
    }
    if x.is_nan() && y.is_nan() {
        return true;
    }
    if x.is_infinite() != y.is_infinite() {
        // one is inf but the other is not
        return false;
    }
    let xi: i32 = unsafe { core::intrinsics::transmute(x) };
    let yi: i32 = unsafe { core::intrinsics::transmute(y) };
    if (xi < 0) != (yi < 0) {
        // different sign
        return false;
    }
    let ulps = (xi - yi).abs();
    ulps <= 1
}

#[cfg(test)]
pub fn equali32(x: i32, y: i32) -> bool {
    let ulps = (x - y).abs();
    ulps <= 1
}
