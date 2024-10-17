use std::fmt;

/// Common types and methods for floating point numbers.
pub trait Float: Copy + fmt::Display + fmt::Debug + PartialEq<Self> {
    type Int: Int<OtherSign = Self::SignedInt, Unsigned = Self::Int>;
    type SignedInt: Int + Int<OtherSign = Self::Int, Unsigned = Self::Int>;

    const BITS: u32;
    fn is_nan(self) -> bool;
    fn to_bits(self) -> Self::Int;
    fn from_bits(bits: Self::Int) -> Self;
    fn signum(self) -> Self;
}

macro_rules! impl_float {
    ($($fty:ty, $ui:ty, $si:ty;)+) => {
        $(
            impl Float for $fty {
                type Int = $ui;
                type SignedInt = $si;
                const BITS: u32 = <$ui>::BITS;
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
    f32, u32, i32;
    f64, u64, i64;
);

/// Common types and methods for integers.
pub trait Int: Copy + fmt::Display + fmt::Debug + PartialEq<Self> {
    type OtherSign: Int;
    type Unsigned: Int;
    const BITS: u32;
    const SIGNED: bool;

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
                const BITS: u32 = <$ui>::BITS;
                const SIGNED: bool = false;
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
                const BITS: u32 = <$ui>::BITS;
                const SIGNED: bool = true;
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
                format!("{self:#0width$x}", width = ((Self::BITS / 8) + 2) as usize)
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
