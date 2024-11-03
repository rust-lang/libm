//! Helpful numeric operations.

use std::cmp::{max, min};

use libm::support::{CastInto, Float};

use crate::{Int, MinInt};

impl<F> FloatExt for F where F: Float {}

/// Additional float methods that build on the `libm` `Float` trait.
pub trait FloatExt: Float {
    /// The minimum subnormal number.
    const TINY_BITS: Self::Int = Self::Int::ONE;

    /// Increment by one ULP, saturating at infinity.
    fn next_up(self) -> Self {
        let bits = self.to_bits();
        if self.is_nan() || bits == Self::INFINITY.to_bits() {
            return self;
        }

        let abs = self.abs().to_bits();
        let next_bits = if abs == Self::Int::ZERO {
            // Next up from 0 is the smallest subnormal
            Self::TINY_BITS
        } else if bits == abs {
            // Positive: counting up is more positive
            bits + Self::Int::ONE
        } else {
            // Negative: counting down is more positive
            bits - Self::Int::ONE
        };
        Self::from_bits(next_bits)
    }

    /// A faster version of `next_up` when skipping a specified number of bits.
    fn n_up(self, n: Self::Int) -> Self {
        let bits = self.to_bits();
        if self.is_nan() || bits == Self::INFINITY.to_bits() || n == Self::Int::ZERO {
            return self;
        }

        let abs = self.abs().to_bits();
        let is_positive = bits == abs;
        let crosses_zero = !is_positive && n > abs;
        let inf_bits = Self::INFINITY.to_bits();

        let next_bits = if abs == Self::Int::ZERO {
            min(n, inf_bits)
        } else if crosses_zero {
            min(n - abs, inf_bits)
        } else if is_positive {
            // Positive, counting up is more positive but this may overflow
            match bits.checked_add(n) {
                Some(v) if v >= inf_bits => inf_bits,
                Some(v) => v,
                None => inf_bits,
            }
        } else {
            // Negative, counting down is more positive
            bits - n
        };
        Self::from_bits(next_bits)
    }

    /// Decrement by one ULP, saturating at negative infinity.
    fn next_down(self) -> Self {
        let bits = self.to_bits();
        if self.is_nan() || bits == Self::NEG_INFINITY.to_bits() {
            return self;
        }

        let abs = self.abs().to_bits();
        let next_bits = if abs == Self::Int::ZERO {
            // Next up from 0 is the smallest negative subnormal
            Self::TINY_BITS | Self::SIGN_MASK
        } else if bits == abs {
            // Positive: counting down is more negative
            bits - Self::Int::ONE
        } else {
            // Negative: counting up is more negative
            bits + Self::Int::ONE
        };
        Self::from_bits(next_bits)
    }

    /// A faster version of `n_down` when skipping a specified number of bits.
    fn n_down(self, n: Self::Int) -> Self {
        let bits = self.to_bits();
        if self.is_nan() || bits == Self::NEG_INFINITY.to_bits() || n == Self::Int::ZERO {
            return self;
        }

        let abs = self.abs().to_bits();
        let is_positive = bits == abs;
        let crosses_zero = is_positive && n > abs;
        let inf_bits = Self::INFINITY.to_bits();
        let ninf_bits = Self::NEG_INFINITY.to_bits();

        let next_bits = if abs == Self::Int::ZERO {
            min(n, inf_bits) | Self::SIGN_MASK
        } else if crosses_zero {
            min(n - abs, inf_bits) | Self::SIGN_MASK
        } else if is_positive {
            // Positive, counting down is more negative
            bits - n
        } else {
            // Negative, counting up is more negative but this may overflow
            match bits.checked_add(n) {
                Some(v) if v > ninf_bits => ninf_bits,
                Some(v) => v,
                None => ninf_bits,
            }
        };
        Self::from_bits(next_bits)
    }

    /// Return the number of steps between two floats, returning `None` if either input is NaN.
    ///
    /// This is the number of steps needed for `n_up` or `n_down` to step between values.
    /// Infinities are treated the same.
    fn ulp_between(self, other: Self) -> Option<Self::Int> {
        if self.is_nan() || other.is_nan() {
            return None;
        }

        let x_abs = self.abs().to_bits();
        let y_abs = other.abs().to_bits();

        if self.signum() == other.signum() {
            Some(max(x_abs, y_abs) - min(x_abs, y_abs))
        } else {
            Some(x_abs + y_abs)
        }
    }
}

/// An iterator that returns floats with linearly spaced integer representations, which translates
/// to logarithmic spacing of their values.
///
/// Note that this tends to skip negative zero, so that needs to be checked explicitly.
pub fn logspace<F: FloatExt>(start: F, end: F, steps: F::Int) -> impl Iterator<Item = F> {
    assert!(!start.is_nan());
    assert!(!end.is_nan());
    assert!(end >= start);

    let mut steps = steps.checked_sub(F::Int::ONE).expect("`steps` must be at least 2");
    let between = start.ulp_between(end).expect("`start `must be less than `end`");
    let spacing = (between / steps).max(F::Int::ONE);
    steps = steps.min(between); // At maximum, one step per ULP

    let mut x = start;
    (0..=steps.cast()).map(move |_| {
        let ret = x;
        x = x.n_up(spacing);
        ret
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::f8;

    #[test]
    fn test_next_up_down() {
        for i in 0..f8::ALL_LEN {
            let v = f8::ALL[i];

            let down = v.next_down().to_bits();
            let up = v.next_up().to_bits();

            if i == 0 {
                assert_eq!(down, f8::NEG_INFINITY.to_bits(), "{i} next_down({v:#010b})");
            } else {
                let expected =
                    if v == f8::ZERO { 1 | f8::SIGN_MASK } else { f8::ALL[i - 1].to_bits() };
                assert_eq!(down, expected, "{i} next_down({v:#010b})");
            }

            if i == f8::ALL_LEN - 1 {
                assert_eq!(up, f8::INFINITY.to_bits(), "{i} next_up({v:#010b})");
            } else {
                let expected = if v == f8::NEG_ZERO { 1 } else { f8::ALL[i + 1].to_bits() };
                assert_eq!(up, expected, "{i} next_up({v:#010b})");
            }
        }
    }

    #[test]
    fn test_next_up_down_inf_nan() {
        assert_eq!(f8::NEG_INFINITY.next_up().to_bits(), f8::ALL[0].to_bits(),);
        assert_eq!(f8::NEG_INFINITY.next_down().to_bits(), f8::NEG_INFINITY.to_bits(),);
        assert_eq!(f8::INFINITY.next_down().to_bits(), f8::ALL[f8::ALL_LEN - 1].to_bits(),);
        assert_eq!(f8::INFINITY.next_up().to_bits(), f8::INFINITY.to_bits(),);
        assert_eq!(f8::NAN.next_up().to_bits(), f8::NAN.to_bits(),);
        assert_eq!(f8::NAN.next_down().to_bits(), f8::NAN.to_bits(),);
    }

    #[test]
    fn test_n_up_down_quick() {
        assert_eq!(f8::ALL[0].n_up(4).to_bits(), f8::ALL[4].to_bits(),);
        assert_eq!(
            f8::ALL[f8::ALL_LEN - 1].n_down(4).to_bits(),
            f8::ALL[f8::ALL_LEN - 5].to_bits(),
        );

        // Check around zero
        assert_eq!(f8::from_bits(0b0).n_up(7).to_bits(), 0b0_0000_111);
        assert_eq!(f8::from_bits(0b0).n_down(7).to_bits(), 0b1_0000_111);

        // Check across zero
        assert_eq!(f8::from_bits(0b1_0000_111).n_up(8).to_bits(), 0b0_0000_001);
        assert_eq!(f8::from_bits(0b0_0000_111).n_down(8).to_bits(), 0b1_0000_001);
    }

    #[test]
    fn test_n_up_down_one() {
        // Verify that `n_up(1)` and `n_down(1)` are the same as `next_up()` and next_down()`.`
        for i in 0..u8::MAX {
            let v = f8::from_bits(i);
            assert_eq!(v.next_up().to_bits(), v.n_up(1).to_bits());
            assert_eq!(v.next_down().to_bits(), v.n_down(1).to_bits());
        }
    }

    #[test]
    fn test_n_up_down_inf_nan_zero() {
        assert_eq!(f8::NEG_INFINITY.n_up(1).to_bits(), f8::ALL[0].to_bits());
        assert_eq!(f8::NEG_INFINITY.n_up(239).to_bits(), f8::ALL[f8::ALL_LEN - 1].to_bits());
        assert_eq!(f8::NEG_INFINITY.n_up(240).to_bits(), f8::INFINITY.to_bits());
        assert_eq!(f8::NEG_INFINITY.n_down(u8::MAX).to_bits(), f8::NEG_INFINITY.to_bits());

        assert_eq!(f8::INFINITY.n_down(1).to_bits(), f8::ALL[f8::ALL_LEN - 1].to_bits());
        assert_eq!(f8::INFINITY.n_down(239).to_bits(), f8::ALL[0].to_bits());
        assert_eq!(f8::INFINITY.n_down(240).to_bits(), f8::NEG_INFINITY.to_bits());
        assert_eq!(f8::INFINITY.n_up(u8::MAX).to_bits(), f8::INFINITY.to_bits());

        assert_eq!(f8::NAN.n_up(u8::MAX).to_bits(), f8::NAN.to_bits());
        assert_eq!(f8::NAN.n_down(u8::MAX).to_bits(), f8::NAN.to_bits());

        assert_eq!(f8::ZERO.n_down(1).to_bits(), f8::TINY_BITS | f8::SIGN_MASK);
        assert_eq!(f8::NEG_ZERO.n_up(1).to_bits(), f8::TINY_BITS);
    }

    /// True if the specified range of `f8::ALL` includes both +0 and -0
    fn crossed_zero(start: usize, end: usize) -> bool {
        let crossed = &f8::ALL[start..=end];
        crossed.iter().any(|f| f8::eq_repr(*f, f8::ZERO))
            && crossed.iter().any(|f| f8::eq_repr(*f, f8::NEG_ZERO))
    }

    #[test]
    fn test_n_up_down() {
        for i in 0..f8::ALL_LEN {
            let v = f8::ALL[i];

            for n in 0..f8::ALL_LEN {
                let down = v.n_down(n as u8).to_bits();
                let up = v.n_up(n as u8).to_bits();

                if let Some(down_exp_idx) = i.checked_sub(n) {
                    // No overflow
                    let mut expected = f8::ALL[down_exp_idx].to_bits();
                    if n >= 1 && crossed_zero(down_exp_idx, i) {
                        // If both -0 and +0 are included, we need to adjust our expected value
                        match down_exp_idx.checked_sub(1) {
                            Some(v) => expected = f8::ALL[v].to_bits(),
                            // Saturate to -inf if we are out of values
                            None => expected = f8::NEG_INFINITY.to_bits(),
                        }
                    }
                    assert_eq!(down, expected, "{i} {n} n_down({v:#010b})");
                } else {
                    // Overflow to -inf
                    assert_eq!(down, f8::NEG_INFINITY.to_bits(), "{i} {n} n_down({v:#010b})");
                }

                let mut up_exp_idx = i + n;
                if up_exp_idx < f8::ALL_LEN {
                    // No overflow
                    if n >= 1 && up_exp_idx < f8::ALL_LEN && crossed_zero(i, up_exp_idx) {
                        // If both -0 and +0 are included, we need to adjust our expected value
                        up_exp_idx += 1;
                    }

                    let expected = if up_exp_idx >= f8::ALL_LEN {
                        f8::INFINITY.to_bits()
                    } else {
                        f8::ALL[up_exp_idx].to_bits()
                    };

                    assert_eq!(up, expected, "{i} {n} n_up({v:#010b})");
                } else {
                    // Overflow to +inf
                    assert_eq!(up, f8::INFINITY.to_bits(), "{i} {n} n_up({v:#010b})");
                }
            }
        }
    }

    #[test]
    fn test_ulp_between() {
        for i in 0..f8::ALL_LEN {
            for j in 0..f8::ALL_LEN {
                let x = f8::ALL[i];
                let y = f8::ALL[j];
                let ulp = x.ulp_between(y).unwrap();
                let make_msg = || format!("i: {i} j: {j} x: {x:b} y: {y:b} ulp {ulp}");

                let i_low = min(i, j);
                let i_hi = max(i, j);
                let mut expected = u8::try_from(i_hi - i_low).unwrap();
                if crossed_zero(i_low, i_hi) {
                    println!("sub");
                    expected -= 1;
                }

                assert_eq!(ulp, expected, "{}", make_msg());

                // Skip if either are zero since `next_{up,down}` will count over it
                let either_zero = x == f8::ZERO || y == f8::ZERO;
                if x < y && !either_zero {
                    assert_eq!(x.n_up(ulp).to_bits(), y.to_bits(), "{}", make_msg());
                    assert_eq!(y.n_down(ulp).to_bits(), x.to_bits(), "{}", make_msg());
                } else if !either_zero {
                    assert_eq!(y.n_up(ulp).to_bits(), x.to_bits(), "{}", make_msg());
                    assert_eq!(x.n_down(ulp).to_bits(), y.to_bits(), "{}", make_msg());
                }
            }
        }
    }

    #[test]
    fn test_ulp_between_inf_nan_zero() {
        assert_eq!(f8::NEG_INFINITY.ulp_between(f8::INFINITY).unwrap(), f8::ALL_LEN as u8);
        assert_eq!(f8::INFINITY.ulp_between(f8::NEG_INFINITY).unwrap(), f8::ALL_LEN as u8);
        assert_eq!(
            f8::NEG_INFINITY.ulp_between(f8::ALL[f8::ALL_LEN - 1]).unwrap(),
            f8::ALL_LEN as u8 - 1
        );
        assert_eq!(f8::INFINITY.ulp_between(f8::ALL[0]).unwrap(), f8::ALL_LEN as u8 - 1);

        assert_eq!(f8::ZERO.ulp_between(f8::NEG_ZERO).unwrap(), 0);
        assert_eq!(f8::NAN.ulp_between(f8::ZERO), None);
        assert_eq!(f8::ZERO.ulp_between(f8::NAN), None);
    }

    #[test]
    fn test_logspace() {
        let ls: Vec<_> = logspace(f8::from_bits(0x0), f8::from_bits(0x4), 3).collect();
        let exp = [f8::from_bits(0x0), f8::from_bits(0x2), f8::from_bits(0x4)];
        assert_eq!(ls, exp);

        let ls: Vec<_> = logspace(f8::from_bits(0x0), f8::from_bits(0x3), 10).collect();
        let exp = [f8::from_bits(0x0), f8::from_bits(0x1), f8::from_bits(0x2), f8::from_bits(0x3)];
        assert_eq!(ls, exp);
    }
}
