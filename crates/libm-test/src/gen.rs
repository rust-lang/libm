//! Different generators that can create random or systematic bit patterns.

use crate::GenerateInput;
pub mod random;

/// Helper type to turn any reusable input into a generator.
#[derive(Clone, Debug, Default)]
pub struct CachedInput {
    #[cfg(f16_enabled)]
    pub inputs_f16: Vec<(f16, f16, f16)>,
    pub inputs_f32: Vec<(f32, f32, f32)>,
    pub inputs_f64: Vec<(f64, f64, f64)>,
    #[cfg(f128_enabled)]
    pub inputs_f128: Vec<(f128, f128, f128)>,
    pub inputs_i32: Vec<(i32, i32, i32)>,
}

#[cfg(f16_enabled)]
impl GenerateInput<(f16,)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f16,)> {
        self.inputs_f16.iter().map(|f| (f.0,))
    }
}

#[cfg(f16_enabled)]
impl GenerateInput<(f16, f16)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f16, f16)> {
        self.inputs_f16.iter().map(|f| (f.0, f.1))
    }
}

impl GenerateInput<(f32,)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f32,)> {
        self.inputs_f32.iter().map(|f| (f.0,))
    }
}

impl GenerateInput<(f32, f32)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f32, f32)> {
        self.inputs_f32.iter().map(|f| (f.0, f.1))
    }
}

impl GenerateInput<(i32, f32)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (i32, f32)> {
        self.inputs_i32.iter().zip(self.inputs_f32.iter()).map(|(i, f)| (i.0, f.0))
    }
}

impl GenerateInput<(f32, i32)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f32, i32)> {
        GenerateInput::<(i32, f32)>::get_cases(self).map(|(i, f)| (f, i))
    }
}

impl GenerateInput<(f32, f32, f32)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f32, f32, f32)> {
        self.inputs_f32.iter().copied()
    }
}

impl GenerateInput<(f64,)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f64,)> {
        self.inputs_f64.iter().map(|f| (f.0,))
    }
}

impl GenerateInput<(f64, f64)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f64, f64)> {
        self.inputs_f64.iter().map(|f| (f.0, f.1))
    }
}

impl GenerateInput<(i32, f64)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (i32, f64)> {
        self.inputs_i32.iter().zip(self.inputs_f64.iter()).map(|(i, f)| (i.0, f.0))
    }
}

impl GenerateInput<(f64, i32)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f64, i32)> {
        GenerateInput::<(i32, f64)>::get_cases(self).map(|(i, f)| (f, i))
    }
}

impl GenerateInput<(f64, f64, f64)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f64, f64, f64)> {
        self.inputs_f64.iter().copied()
    }
}

#[cfg(f128_enabled)]
impl GenerateInput<(f128,)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f128,)> {
        self.inputs_f128.iter().map(|f| (f.0,))
    }
}

#[cfg(f128_enabled)]
impl GenerateInput<(f128, f128)> for CachedInput {
    fn get_cases(&self) -> impl Iterator<Item = (f128, f128)> {
        self.inputs_f128.iter().map(|f| (f.0, f.1))
    }
}
