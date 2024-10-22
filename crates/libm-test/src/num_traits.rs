use std::fmt;
use std::ops;

/// Common types and methods for floating point numbers.
pub trait Float: Copy + fmt::Display + fmt::Debug + PartialEq<Self> {
    type Int: Int<OtherSign = Self::SignedInt, Unsigned = Self::Int>;
    type SignedInt: Int + Int<OtherSign = Self::Int, Unsigned = Self::Int>;

    /// The bitwidth of the float type
    const BITS: u32;

    /// The bitwidth of the significand
    const SIGNIFICAND_BITS: u32;

    /// The bitwidth of the exponent
    const EXPONENT_BITS: u32 = Self::BITS - Self::SIGNIFICAND_BITS - 1;

    /// The saturated value of the exponent (infinite representation), in the rightmost postiion.
    const EXPONENT_MAX: u32 = (1 << Self::EXPONENT_BITS) - 1;

    /// The exponent bias value
    const EXPONENT_BIAS: u32 = Self::EXPONENT_MAX >> 1;

    /// A mask for the sign bit
    const SIGN_MASK: Self::Int;

    /// A mask for the significand
    const SIGNIFICAND_MASK: Self::Int;

    /// The implicit bit of the float format
    const IMPLICIT_BIT: Self::Int;

    /// A mask for the exponent
    const EXPONENT_MASK: Self::Int;

    fn is_nan(self) -> bool;
    fn to_bits(self) -> Self::Int;
    fn from_bits(bits: Self::Int) -> Self;
    fn signum(self) -> Self;
    /// Constructs a `Self` from its parts. Inputs are treated as bits and shifted into position.
    fn from_parts(sign: bool, exponent: Self::Int, significand: Self::Int) -> Self;
}

macro_rules! impl_float {
    ($($fty:ty, $ui:ty, $si:ty, $significand_bits:expr;)+) => {
        $(
            impl Float for $fty {
                type Int = $ui;
                type SignedInt = $si;

                const BITS: u32 = <$ui>::BITS;
                const SIGNIFICAND_BITS: u32 = $significand_bits;

                const SIGN_MASK: Self::Int = 1 << (Self::BITS - 1);
                const SIGNIFICAND_MASK: Self::Int = (1 << Self::SIGNIFICAND_BITS) - 1;
                const IMPLICIT_BIT: Self::Int = 1 << Self::SIGNIFICAND_BITS;
                const EXPONENT_MASK: Self::Int = !(Self::SIGN_MASK | Self::SIGNIFICAND_MASK);

                fn is_nan(self) -> bool {
                    self.is_nan()
                }
                fn to_bits(self) -> Self::Int {
                    self.to_bits()
                }
                fn from_bits(bits: Self::Int) -> Self {
                    Self::from_bits(bits)
                }
                fn signum(self) -> Self {
                    self.signum()
                }
                fn from_parts(sign: bool, exponent: Self::Int, significand: Self::Int) -> Self {
                    Self::from_bits(
                        ((sign as Self::Int) << (Self::BITS - 1))
                            | ((exponent << Self::SIGNIFICAND_BITS) & Self::EXPONENT_MASK)
                            | (significand & Self::SIGNIFICAND_MASK),
                    )
                }
            }

            impl Hex for $fty {
                fn hex(self) -> String {
                    self.to_bits().hex()
                }
            }
        )+
    }
}

impl_float!(
    f32, u32, i32, 23;
    f64, u64, i64, 52;
);

/// Common types and methods for integers.
pub trait Int:
    Copy
    + fmt::Display
    + fmt::Debug
    + PartialEq<Self>
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Shr<u32, Output = Self>
    + ops::Shl<u32, Output = Self>
    + 'static
{
    /// Type with the same width but other signedness
    type OtherSign: Int;
    /// Unsigned version of Self
    type Unsigned: Int;

    /// If `Self` is a signed integer
    const SIGNED: bool;

    /// The bitwidth of the int type
    const BITS: u32;

    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;

    fn signed(self) -> <Self::Unsigned as Int>::OtherSign;
    fn unsigned(self) -> Self::Unsigned;
    fn checked_sub(self, other: Self) -> Option<Self>;
    fn abs(self) -> Self;
}

macro_rules! impl_int {
    ($($ui:ty, $si:ty ;)+) => {
        $(
            impl Int for $ui {
                type OtherSign = $si;
                type Unsigned = Self;

                const SIGNED: bool = false;
                const BITS: u32 = <$ui>::BITS;
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;

                fn signed(self) -> Self::OtherSign {
                    self as $si
                }
                fn unsigned(self) -> Self {
                    self
                }
                fn checked_sub(self, other: Self) -> Option<Self> {
                    self.checked_sub(other)
                }
                fn abs(self) -> Self {
                    unimplemented!()
                }
            }

            impl Int for $si {
                type OtherSign = $ui;
                type Unsigned = $ui;

                const SIGNED: bool = true;
                const BITS: u32 = <$ui>::BITS;
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;

                fn signed(self) -> Self {
                    self
                }
                fn unsigned(self) -> $ui {
                    self as $ui
                }
                fn checked_sub(self, other: Self) -> Option<Self> {
                    self.checked_sub(other)
                }
                fn abs(self) -> Self {
                    self.abs()
                }
            }

            impl_int!(@for_both $si);
            impl_int!(@for_both $ui);

        )+
    };

    (@for_both $ty:ty) => {
        impl Hex for $ty {
            fn hex(self) -> String {
                format!("{self:#0width$x}", width = ((Self::BITS / 4) + 2) as usize)
            }
        }

        impl<Input: Hex + fmt::Debug> $crate::CheckOutput<Input> for $ty {
            fn validate<'a>(
                self,
                expected: Self,
                input: Input,
                _ctx: &$crate::CheckCtx,
            ) -> anyhow::Result<()> {
                anyhow::ensure!(
                    self == expected,
                    "\
                    \n    input:    {input:?} {ibits}\
                    \n    expected: {expected:<22?} {expbits}\
                    \n    actual:   {self:<22?} {actbits}\
                    ",
                    actbits = self.hex(),
                    expbits = expected.hex(),
                    ibits = input.hex(),
                );

                Ok(())
            }
        }
    }
}

impl_int!(
    u32, i32;
    u64, i64;
);

/// A helper trait to print something as hex with the correct number of nibbles, e.g. a `u32`
/// will always print with `0x` followed by 8 digits.
///
/// This is only used for printing errors so allocating is okay.
pub trait Hex: Copy {
    fn hex(self) -> String;
}

impl<T1> Hex for (T1,)
where
    T1: Hex,
{
    fn hex(self) -> String {
        format!("({},)", self.0.hex())
    }
}

impl<T1, T2> Hex for (T1, T2)
where
    T1: Hex,
    T2: Hex,
{
    fn hex(self) -> String {
        format!("({}, {})", self.0.hex(), self.1.hex())
    }
}

impl<T1, T2, T3> Hex for (T1, T2, T3)
where
    T1: Hex,
    T2: Hex,
    T3: Hex,
{
    fn hex(self) -> String {
        format!("({}, {}, {})", self.0.hex(), self.1.hex(), self.2.hex())
    }
}
