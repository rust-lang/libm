#![no_std]
#![feature(lang_items)]
#![feature(core_intrinsics)]
use core::intrinsics;
use core::panic::PanicInfo;

mod lib_f32;
mod lib_f64;
mod lib_fenv;
mod lib_long_double;
mod musl_missing;
mod new_lib;

pub use lib_f32::*;
pub use lib_f64::*;
pub use lib_fenv::*;
pub use lib_long_double::*;

pub use musl_missing::*;
pub use new_lib::*;

#[no_mangle]
pub static mut signgam: i32 = 0;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
   intrinsics::abort()
}

/*
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    extern "Rust" {
        #[link_name = "\nerror(panic-never): your program contains at least one panicking branch"]
        fn undefined() -> !;
    }

    unsafe { undefined() }
}
*/

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
