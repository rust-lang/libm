use super::lgammaf_r;

pub extern "C" fn lgammaf(x: f32) -> f32 {
    lgammaf_r(x).0
}
