//! Traits for generating ranges of floating-point values

pub trait Toward: Sized {
    /// Generates `len` minimal incremental steps from self to other.
    ///
    /// Other is often never reached.
    fn toward(self, other: Self, len: usize) -> Vec<Self>;
}

pub trait Distribute: Sized {
    /// Distributes `len` values in range `[self, other]`.
    fn distribute(self, other: Self, len: usize) -> Vec<Self>;
}

macro_rules! impl_f {
    ($float_ty:ident, $toward_fn:path) => {
        impl Toward for $float_ty {
            fn toward(self, other: Self, len: usize) -> Vec<Self> {
                let mut vec = Vec::with_capacity(len);
                let mut current = self;
                vec.push(self);
                for _ in 0..=len {
                    current = $toward_fn(current, other as _);
                    vec.push(self);
                    if current.to_bits() == other.to_bits() {
                        break;
                    }
                }
                vec
            }
        }
        impl Distribute for $float_ty {
            fn distribute(self, other: Self, mut len: usize) -> Vec<Self> {
                let mut vec = Vec::with_capacity(len + 1);
                vec.push(self);
                // Bresenham's alg:
                let mut x = self;
                while len > 0 {
                    x += (other - self) / (len as Self);
                    len -= 1;
                    vec.push(x);
                }
                vec
            }
        }
    };
}
impl_f!(f32, libm::nextafterf);
impl_f!(f64, libm::nextafter);
