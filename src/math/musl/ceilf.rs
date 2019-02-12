use core::f32;

#[inline]
pub fn ceilf(x: f32) -> f32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f32.ceil` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::ceilf32(x) }
        }
    }
    let mut ui = x.to_bits();
    let e = (((ui >> 23) & 0xff) as i32) - 0x7f;

    if e >= 23 {
        return x;
    }
    if e >= 0 {
        let m = 0x_007f_ffff >> e;
        if (ui & m) == 0 {
            return x;
        }
        force_eval!(x + f32::from_bits(0x_7b80_0000));
        if ui >> 31 == 0 {
            ui += m;
        }
        ui &= !m;
    } else {
        force_eval!(x + f32::from_bits(0x_7b80_0000));
        if ui >> 31 != 0 {
            return -0.;
        } else if ui << 1 != 0 {
            return 1.;
        }
    }
    f32::from_bits(ui)
}
