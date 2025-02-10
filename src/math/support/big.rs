//! Integers used for wide operations, larger than `u128`.

#[cfg(test)]
mod tests;

use core::ops;

use super::{DInt, HInt, Int, MinInt};

const U128_LO_MASK: u128 = u64::MAX as u128;

/// A 256-bit unsigned integer represented as two 128-bit native-endian limbs.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct u256 {
    pub lo: u128,
    pub hi: u128,
}

impl u256 {
    #[cfg(any(test, feature = "unstable-public-internals"))]
    pub const MAX: Self = Self { lo: u128::MAX, hi: u128::MAX };

    /// Reinterpret as a signed integer
    pub fn signed(self) -> i256 {
        i256 { lo: self.lo, hi: self.hi }
    }
}

/// A 256-bit signed integer represented as two 128-bit native-endian limbs.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct i256 {
    pub lo: u128,
    pub hi: u128,
}

impl i256 {
    /// Reinterpret as an unsigned integer
    #[cfg(any(test, feature = "unstable-public-internals"))]
    pub fn unsigned(self) -> u256 {
        u256 { lo: self.lo, hi: self.hi }
    }
}

impl MinInt for u256 {
    type OtherSign = i256;

    type Unsigned = u256;

    const SIGNED: bool = false;
    const BITS: u32 = 256;
    const ZERO: Self = Self { lo: 0, hi: 0 };
    const ONE: Self = Self { lo: 1, hi: 0 };
    const MIN: Self = Self { lo: 0, hi: 0 };
    const MAX: Self = Self { lo: u128::MAX, hi: u128::MAX };
}

impl MinInt for i256 {
    type OtherSign = u256;

    type Unsigned = u256;

    const SIGNED: bool = false;
    const BITS: u32 = 256;
    const ZERO: Self = Self { lo: 0, hi: 0 };
    const ONE: Self = Self { lo: 1, hi: 0 };
    const MIN: Self = Self { lo: 0, hi: 1 << 127 };
    const MAX: Self = Self { lo: u128::MAX, hi: u128::MAX << 1 };
}

macro_rules! impl_common {
    ($ty:ty) => {
        impl ops::BitOr for $ty {
            type Output = Self;

            fn bitor(mut self, rhs: Self) -> Self::Output {
                self.lo |= rhs.lo;
                self.hi |= rhs.hi;
                self
            }
        }

        impl ops::Not for $ty {
            type Output = Self;

            fn not(mut self) -> Self::Output {
                self.lo = !self.lo;
                self.hi = !self.hi;
                self
            }
        }

        impl ops::Shl<u32> for $ty {
            type Output = Self;

            fn shl(self, _rhs: u32) -> Self::Output {
                unimplemented!("only used to meet trait bounds")
            }
        }
    };
}

impl_common!(i256);
impl_common!(u256);

impl ops::Add<Self> for u256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (lo, carry) = self.lo.overflowing_add(rhs.lo);
        let hi = self.hi.wrapping_add(carry as u128).wrapping_add(rhs.hi);

        Self { lo, hi }
    }
}

impl ops::Shr<u32> for u256 {
    type Output = Self;

    fn shr(mut self, rhs: u32) -> Self::Output {
        debug_assert!(rhs < Self::BITS, "attempted to shift right with overflow");

        /* NB: LLVM produces significantly better codegen with a manual `u64`
         * array-based shift compared to a u128 `>>`-based version. */

        // Store input to an array with 256 bits of zero beyond the msb
        let mut xarr = [0u8; size_of::<u256>() * 2];
        xarr[..16].copy_from_slice(&self.lo.to_le_bytes());
        xarr[16..32].copy_from_slice(&self.hi.to_le_bytes());

        // Maximum shift is 256, all other values are 0. `x >> 256 = 0` by
        // default with this algorithm.
        let shift = rhs.min(256);

        // Split shift into a coarse bytewise shift (done via array access) and
        // a fine shift (done with bit shifts).
        let byteshift = (shift as usize / 64) * 8;
        let bitshift = shift % 64;

        // Apply the coarse shift by accessing within the array, possibly
        let mut r0b = [0u8; 8];
        let mut r1b = [0u8; 8];
        let mut r2b = [0u8; 8];
        let mut r3b = [0u8; 8];
        r0b.copy_from_slice(&xarr[byteshift..(8 + byteshift)]);
        r1b.copy_from_slice(&xarr[(8 + byteshift)..(16 + byteshift)]);
        r2b.copy_from_slice(&xarr[(16 + byteshift)..(24 + byteshift)]);
        r3b.copy_from_slice(&xarr[(24 + byteshift)..(32 + byteshift)]);
        let mut r0 = u64::from_le_bytes(r0b);
        let mut r1 = u64::from_le_bytes(r1b);
        let mut r2 = u64::from_le_bytes(r2b);
        let mut r3 = u64::from_le_bytes(r3b);

        // Apply the fine shifts
        r0 >>= bitshift;
        r0 |= r1.checked_shl(64 - bitshift).unwrap_or(0);
        r1 >>= bitshift;
        r1 |= r2.checked_shl(64 - bitshift).unwrap_or(0);
        r2 >>= bitshift;
        r2 |= r3.checked_shl(64 - bitshift).unwrap_or(0);
        r3 >>= bitshift;

        // Transmute <2 x u64> to u128 via arrays, then store
        let mut lo = [0u8; 16];
        let mut hi = [0u8; 16];
        lo[..8].copy_from_slice(&r0.to_le_bytes());
        lo[8..].copy_from_slice(&r1.to_le_bytes());
        hi[..8].copy_from_slice(&r2.to_le_bytes());
        hi[8..].copy_from_slice(&r3.to_le_bytes());
        self.lo = u128::from_le_bytes(lo);
        self.hi = u128::from_le_bytes(hi);

        self
    }
}

impl HInt for u128 {
    type D = u256;

    fn widen(self) -> Self::D {
        u256 { lo: self, hi: 0 }
    }

    fn zero_widen(self) -> Self::D {
        self.widen()
    }

    fn zero_widen_mul(self, rhs: Self) -> Self::D {
        let l0 = self & U128_LO_MASK;
        let l1 = rhs & U128_LO_MASK;
        let h0 = self >> 64;
        let h1 = rhs >> 64;

        let p_ll: u128 = l0.overflowing_mul(l1).0;
        let p_lh: u128 = l0.overflowing_mul(h1).0;
        let p_hl: u128 = h0.overflowing_mul(l1).0;
        let p_hh: u128 = h0.overflowing_mul(h1).0;

        let s0 = p_hl + (p_ll >> 64);
        let s1 = (p_ll & U128_LO_MASK) + (s0 << 64);
        let s2 = p_lh + (s1 >> 64);

        let lo = (p_ll & U128_LO_MASK) + (s2 << 64);
        let hi = p_hh + (s0 >> 64) + (s2 >> 64);

        u256 { lo, hi }
    }

    fn widen_mul(self, rhs: Self) -> Self::D {
        self.zero_widen_mul(rhs)
    }

    fn widen_hi(self) -> Self::D {
        self.widen() << <Self as MinInt>::BITS
    }
}

impl HInt for i128 {
    type D = i256;

    fn widen(self) -> Self::D {
        let mut ret = self.unsigned().zero_widen().signed();
        if self.is_negative() {
            ret.hi = u128::MAX;
        }
        ret
    }

    fn zero_widen(self) -> Self::D {
        self.unsigned().zero_widen().signed()
    }

    fn zero_widen_mul(self, rhs: Self) -> Self::D {
        self.unsigned().zero_widen_mul(rhs.unsigned()).signed()
    }

    fn widen_mul(self, _rhs: Self) -> Self::D {
        unimplemented!("signed i128 widening multiply is not used")
    }

    fn widen_hi(self) -> Self::D {
        self.widen() << <Self as MinInt>::BITS
    }
}

impl DInt for u256 {
    type H = u128;

    fn lo(self) -> Self::H {
        self.lo
    }

    fn hi(self) -> Self::H {
        self.hi
    }
}

impl DInt for i256 {
    type H = i128;

    fn lo(self) -> Self::H {
        self.lo as i128
    }

    fn hi(self) -> Self::H {
        self.hi as i128
    }
}
