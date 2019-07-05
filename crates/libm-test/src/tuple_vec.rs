//! Trait for tuples of vectors

/// Access to tuple elements of a tuple of vectors
pub trait TupleVec {
    type Output;
    fn get(&self, i: usize) -> Self::Output;
}

macro_rules! impl_tuple_vec {
    (($($arg_tys:ty),*): $self_:ident: $($xs:expr),*)  => {
        impl TupleVec for ($(Vec<$arg_tys>,)+) {
            type Output = ($($arg_tys,)+);
            fn get(&self, i: usize) -> Self::Output {
                let $self_ = self;
                ($($xs[i],)*)
            }
        }
    };
}

impl_tuple_vec!((f32): x: x.0);
impl_tuple_vec!((f64): x: x.0);
impl_tuple_vec!((f32, f32): x: x.0, x.1);
impl_tuple_vec!((f32, f64): x: x.0, x.1);
impl_tuple_vec!((f64, f64): x: x.0, x.1);
impl_tuple_vec!((f64, i32): x: x.0, x.1);
impl_tuple_vec!((f32, i32): x: x.0, x.1);
impl_tuple_vec!((i32, f64): x: x.0, x.1);
impl_tuple_vec!((i32, f32): x: x.0, x.1);
impl_tuple_vec!((f32, f32, f32): x: x.0, x.1, x.2);
impl_tuple_vec!((f64, f64, f64): x: x.0, x.1, x.2);
