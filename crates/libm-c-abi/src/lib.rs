mod lib_f32;
mod lib_f64;
mod lib_fenv;
mod lib_long_double;

pub use lib_f32::*;
pub use lib_f64::*;
pub use lib_fenv::*;
pub use lib_long_double::*;

#[no_mangle]
pub static mut signgam: i32 = 0;
