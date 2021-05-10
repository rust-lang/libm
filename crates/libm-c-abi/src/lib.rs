#![no_std]
#![feature(lang_items)]

mod lib_f32;
mod lib_f64;
mod lib_fenv;
mod lib_long_double;
mod musl_missing;

pub use lib_f32::*;
pub use lib_f64::*;
pub use lib_fenv::*;
pub use lib_long_double::*;

pub use musl_missing::*;

#[no_mangle]
pub static mut signgam: i32 = 0;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
