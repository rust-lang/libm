use std::{ffi::c_int, fmt};

// List of all files present in libm's source
include!(concat!(env!("OUT_DIR"), "/all_files.rs"));

pub trait Float: Copy + fmt::Display + fmt::Debug + PartialEq<Self> {
    type Int: Int<OtherSign = Self::SignedInt, Unsigned = Self::Int>;
    type SignedInt: Int + Int<OtherSign = Self::Int, Unsigned = Self::Int>;

    const BITS: u32;
    fn is_nan(self) -> bool;
    fn to_bits(self) -> Self::Int;
    fn signum(self) -> Self;
}

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

impl Float for f32 {
    type Int = u32;
    type SignedInt = i32;
    const BITS: u32 = 32;
    fn is_nan(self) -> bool {
        self.is_nan()
    }
    fn to_bits(self) -> Self::Int {
        self.to_bits()
    }
    fn signum(self) -> Self {
        self.signum()
    }
}

impl Float for f64 {
    type Int = u64;
    type SignedInt = i64;
    const BITS: u32 = 64;
    fn is_nan(self) -> bool {
        self.is_nan()
    }
    fn to_bits(self) -> Self::Int {
        self.to_bits()
    }
    fn signum(self) -> Self {
        self.signum()
    }
}

impl Int for u32 {
    type OtherSign = i32;
    type Unsigned = Self;
    const BITS: u32 = 32;
    const SIGNED: bool = false;
    fn signed(self) -> i32 {
        self as i32
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

impl Int for u64 {
    type OtherSign = i64;
    type Unsigned = Self;
    const BITS: u32 = 64;
    const SIGNED: bool = false;
    fn signed(self) -> i64 {
        self as i64
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

impl Int for i32 {
    type OtherSign = u32;
    type Unsigned = u32;
    const BITS: u32 = 32;
    const SIGNED: bool = true;
    fn signed(self) -> Self {
        self
    }
    fn unsigned(self) -> u32 {
        self as u32
    }
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub(other)
    }
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Int for i64 {
    type OtherSign = u64;
    type Unsigned = u64;
    const BITS: u32 = 64;
    const SIGNED: bool = true;
    fn signed(self) -> Self {
        self
    }
    fn unsigned(self) -> u64 {
        self as u64
    }
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub(other)
    }
    fn abs(self) -> Self {
        self.abs()
    }
}

pub trait TupleCall<F>: fmt::Debug {
    type Output;
    fn call(self, f: F) -> Self::Output;
}

macro_rules! impl_tupl_call {
    ($( ($($argty:ty),*) -> $ret:ty; )+) => {
        $(
            impl TupleCall<fn( $($argty),* ) -> $ret> for ( $($argty,)* ) {
                type Output = $ret;

                fn call(self, f: fn($($argty),*) -> $ret) -> Self::Output {
                    impl_tupl_call!(@call f, self, $($argty),*)
                }
            }
        )*
    };

    (@call $f:ident, $this:ident, $a1:ty, $a2:ty, $a3:ty) => {
        $f($this.0, $this.1, $this.2)
    };

    (@call $f:ident, $this:ident, $a1:ty, $a2:ty) => {
        $f($this.0, $this.1)
    };

    (@call $f:ident, $this:ident, $a1:ty) => {
        $f($this.0)
    };
}

impl_tupl_call! {
    (f32) -> f32;
    (f64) -> f64;
    (f32) -> i32;
    (f64) -> i32;
    (f32, f32) -> f32;
    (f64, f64) -> f64;
    (f32, i32) -> f32;
    (f64, i32) -> f64;
    (i32, f32) -> f32;
    (i32, f64) -> f64;
    (f32, f32, f32) -> f32;
    (f64, f64, f64) -> f64;
    (f32) -> (f32, f32);
    (f64) -> (f64, f64);
    (f32) -> (f32, c_int);
    (f64) -> (f64, c_int);
    (f32, f32) -> (f32, c_int);
    (f64, f64) -> (f64, c_int);
}

impl TupleCall<fn(f32, &mut c_int) -> f32> for (f32,) {
    type Output = (f32, c_int);

    fn call(self, f: fn(f32, &mut c_int) -> f32) -> Self::Output {
        let mut iret = 0;
        let fret = f(self.0, &mut iret);
        (fret, iret)
    }
}
impl TupleCall<fn(f64, &mut c_int) -> f64> for (f64,) {
    type Output = (f64, c_int);

    fn call(self, f: fn(f64, &mut c_int) -> f64) -> Self::Output {
        let mut iret = 0;
        let fret = f(self.0, &mut iret);
        (fret, iret)
    }
}
impl TupleCall<fn(f32, &mut f32) -> f32> for (f32,) {
    type Output = (f32, f32);

    fn call(self, f: fn(f32, &mut f32) -> f32) -> Self::Output {
        let mut ret2 = 0.0;
        let ret1 = f(self.0, &mut ret2);
        (ret1, ret2)
    }
}
impl TupleCall<fn(f64, &mut f64) -> f64> for (f64,) {
    type Output = (f64, f64);

    fn call(self, f: fn(f64, &mut f64) -> f64) -> Self::Output {
        let mut ret2 = 0.0;
        let ret1 = f(self.0, &mut ret2);
        (ret1, ret2)
    }
}
impl TupleCall<fn(f32, f32, &mut c_int) -> f32> for (f32, f32) {
    type Output = (f32, c_int);

    fn call(self, f: fn(f32, f32, &mut c_int) -> f32) -> Self::Output {
        let mut iret = 0;
        let fret = f(self.0, self.1, &mut iret);
        (fret, iret)
    }
}
impl TupleCall<fn(f64, f64, &mut c_int) -> f64> for (f64, f64) {
    type Output = (f64, c_int);

    fn call(self, f: fn(f64, f64, &mut c_int) -> f64) -> Self::Output {
        let mut iret = 0;
        let fret = f(self.0, self.1, &mut iret);
        (fret, iret)
    }
}
impl TupleCall<fn(f32, &mut f32, &mut f32)> for (f32,) {
    type Output = (f32, f32);

    fn call(self, f: fn(f32, &mut f32, &mut f32)) -> Self::Output {
        let mut ret1 = 0.0;
        let mut ret2 = 0.0;
        f(self.0, &mut ret1, &mut ret2);
        (ret1, ret2)
    }
}
impl TupleCall<fn(f64, &mut f64, &mut f64)> for (f64,) {
    type Output = (f64, f64);

    fn call(self, f: fn(f64, &mut f64, &mut f64)) -> Self::Output {
        let mut ret1 = 0.0;
        let mut ret2 = 0.0;
        f(self.0, &mut ret1, &mut ret2);
        (ret1, ret2)
    }
}

#[derive(Clone, Debug, Default)]
pub struct TestCases {
    pub inputs_f32: Vec<(f32,)>,
    pub inputs_f64: Vec<(f64,)>,
    pub inputs_f32_f32: Vec<(f32, f32)>,
    pub inputs_f64_f64: Vec<(f64, f64)>,
    pub inputs_i32_f32: Vec<(i32, f32)>,
    pub inputs_i32_f64: Vec<(i32, f64)>,
    pub inputs_f32_i32: Vec<(f32, i32)>,
    pub inputs_f64_i32: Vec<(f64, i32)>,
    pub inputs_f32_f32_f32: Vec<(f32, f32, f32)>,
    pub inputs_f64_f64_f64: Vec<(f64, f64, f64)>,
}

pub trait GetVal: Sized {
    type Case;
    fn get_cases(all: &TestCases) -> &[Self::Case];
}

impl GetVal for (f32,) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_f32
    }
}
impl GetVal for (f32, f32) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_f32_f32
    }
}
impl GetVal for (i32, f32) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_i32_f32
    }
}
impl GetVal for (f32, i32) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_f32_i32
    }
}
impl GetVal for (f64, i32) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_f64_i32
    }
}
impl GetVal for (f32, f32, f32) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_f32_f32_f32
    }
}
impl GetVal for (f64,) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_f64
    }
}
impl GetVal for (f64, f64) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_f64_f64
    }
}
impl GetVal for (f32, &mut i32) {
    type Case = (f32, i32);
    fn get_cases(all: &TestCases) -> &[Self::Case] {
        &all.inputs_f32_i32
    }
}
impl GetVal for (f64, &mut i32) {
    type Case = (f64, i32);
    fn get_cases(all: &TestCases) -> &[Self::Case] {
        &all.inputs_f64_i32
    }
}
impl GetVal for (i32, f64) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_i32_f64
    }
}
impl GetVal for (f64, f64, f64) {
    type Case = Self;
    fn get_cases(all: &TestCases) -> &[Self] {
        &all.inputs_f64_f64_f64
    }
}

pub trait Hex: Copy {
    fn hex(self) -> String;
}

impl Hex for u32 {
    fn hex(self) -> String {
        format!("{:#010x}", self)
    }
}
impl Hex for u64 {
    fn hex(self) -> String {
        format!("{:#018x}", self)
    }
}
impl Hex for i32 {
    fn hex(self) -> String {
        format!("{:#010x}", self)
    }
}
impl Hex for i64 {
    fn hex(self) -> String {
        format!("{:#018x}", self)
    }
}

impl Hex for f32 {
    fn hex(self) -> String {
        self.to_bits().hex()
    }
}

impl Hex for f64 {
    fn hex(self) -> String {
        self.to_bits().hex()
    }
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

pub trait CheckOutput: Sized {
    fn validate<Input>(self, expected: Self, input: Input, allowed_ulp: u32)
    where
        Input: Hex + fmt::Debug,
        Self: Hex;
}

impl<F: Float> CheckOutput for F
where
    u32: TryFrom<F::SignedInt, Error: fmt::Debug>,
{
    fn validate<Input>(self, expected: Self, input: Input, allowed_ulp: u32)
    where
        Input: Hex + fmt::Debug,
        Self: Hex,
    {
        let make_msg = || {
            format!(
                "expected {expected:?} crate {self:?} ({expbits}, {actbits}) input {input:?} ({ibits})",
                expbits = expected.hex(),
                actbits = self.hex(),
                ibits = input.hex()
           )
        };

        if self.is_nan() && expected.is_nan() {
            assert_eq!(
                self.to_bits(),
                expected.to_bits(),
                "NaN different bitpatterns: {}",
                make_msg()
            );
            // Nothing else to do if NaNs.
            return;
        } else if self.is_nan() || expected.is_nan() {
            panic!("mismatched NaN: {}", make_msg());
        }

        assert_eq!(
            self.signum(),
            expected.signum(),
            "mismatched signs: {}",
            make_msg()
        );

        let ulp_diff = self
            .to_bits()
            .signed()
            .checked_sub(expected.to_bits().signed())
            .unwrap()
            .abs();

        assert!(
            u32::try_from(ulp_diff).unwrap() <= allowed_ulp,
            "ulp {ulp_diff} > {allowed_ulp}: {}",
            make_msg()
        );
    }
}

impl CheckOutput for i32 {
    fn validate<Input>(self, expected: Self, input: Input, _allowed_ulp: u32)
    where
        Input: Hex + fmt::Debug,
        Self: Hex,
    {
        assert_eq!(
            self,
            expected,
            "expected {expected:?} crate {self:?} ({expbits}, {actbits}) input {input:?} ({ibits})",
            expbits = expected.hex(),
            actbits = self.hex(),
            ibits = input.hex()
        );
    }
}

impl CheckOutput for i64 {
    fn validate<Input>(self, expected: Self, input: Input, _allowed_ulp: u32)
    where
        Input: Hex + fmt::Debug,
        Self: Hex,
    {
        assert_eq!(
            self,
            expected,
            "expected {expected:?} crate {self:?} ({expbits}, {actbits}) input {input:?} ({ibits})",
            expbits = expected.hex(),
            actbits = self.hex(),
            ibits = input.hex()
        );
    }
}

impl CheckOutput for (f32, i32) {
    fn validate<Input>(self, expected: Self, input: Input, allowed_ulp: u32)
    where
        Input: Hex + fmt::Debug,
        Self: Hex,
    {
        self.0.validate(expected.0, input, allowed_ulp);
        self.1.validate(expected.1, input, allowed_ulp);
    }
}
impl CheckOutput for (f64, i32) {
    fn validate<Input>(self, expected: Self, input: Input, allowed_ulp: u32)
    where
        Input: Hex + fmt::Debug,
        Self: Hex,
    {
        self.0.validate(expected.0, input, allowed_ulp);
        self.1.validate(expected.1, input, allowed_ulp);
    }
}
impl CheckOutput for (f32, f32) {
    fn validate<Input>(self, expected: Self, input: Input, allowed_ulp: u32)
    where
        Input: Hex + fmt::Debug,
        Self: Hex,
    {
        self.0.validate(expected.0, input, allowed_ulp);
        self.1.validate(expected.1, input, allowed_ulp);
    }
}
impl CheckOutput for (f64, f64) {
    fn validate<Input>(self, expected: Self, input: Input, allowed_ulp: u32)
    where
        Input: Hex + fmt::Debug,
        Self: Hex,
    {
        self.0.validate(expected.0, input, allowed_ulp);
        self.1.validate(expected.1, input, allowed_ulp);
    }
}
